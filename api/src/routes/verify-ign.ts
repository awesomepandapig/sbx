import express, { Request, Response } from 'express';
import { z } from 'zod';
import { eq } from 'drizzle-orm';
import { createSelectSchema } from 'drizzle-zod';
import { db } from '../db/index';
import { user } from '../db/schema';
import { authenticate } from '../lib/middleware';
import 'dotenv/config';

if (!process.env.HYPIXEL_API_KEY) {
  throw new Error('HYPIXEL_API_KEY not set');
}

const userSelectSchema = createSelectSchema(user);

const ignSchema = z
  .string()
  .trim()
  .regex(/^[a-zA-Z0-9_]{3,16}$/);
const minecraftIdSchema = z
  .string()
  .regex(/^[0-9a-fA-F]{32}$/, 'Invalid Minecraft ID')
  .length(32);

// const ignRequestSchema = z
//   .object({
//     ign: z.string().min(3).max(16),
//   })
//   .strict();

const router = express.Router();

async function getMinecraftId(ign: string): Promise<string> {
  try {
    const url = `https://api.mojang.com/users/profiles/minecraft/${ign}`;
    const response = await fetch(url);
    if (!response.ok) {
      throw new Error('Invalid IGN or API request failed.');
    }
    const data = await response.json();
    minecraftIdSchema.parse(data.id);
    return data.id;
  } catch (error) {
    console.error('Error fetching Minecraft UUID:', error);
    throw new Error('Failed to fetch Minecraft UUID.');
  }
}

async function getHypixelDiscord(uuid: string): Promise<string | null> {
  try {
    const url = `https://api.hypixel.net/v2/player?uuid=${uuid}`;
    const response = await fetch(url, {
      method: 'GET',
      headers: {
        'API-Key': process.env.HYPIXEL_API_KEY as string,
      },
    });
    if (!response.ok) {
      throw new Error('Failed to fetch Hypixel player data.');
    }

    const data = await response.json();
    if (!data.player?.socialMedia?.links?.DISCORD) {
      throw new Error('Hypixel account is not linked to a Discord account.');
    }

    return data.player.socialMedia.links.DISCORD;
  } catch (error) {
    throw new Error(`Failed to fetch Hypixel Discord: ${error}`); // TODO: CHANGE ERROR BEHAVIOR
  }
}

// TODO: Validate body matches expected STRICTLY (no extra json fields)
router.post('/', authenticate, async (req: Request, res: Response) => {
  try {
    const session = res.locals.session;
    ignSchema.parse(req.body.ign);
    const ign = req.body.ign;

    const minecraftId = await getMinecraftId(ign);

    const hypixelDiscordId = await getHypixelDiscord(minecraftId);
    if (!hypixelDiscordId) {
      res
        .status(400)
        .json({
          error:
            'Your Hypixel account is not linked to a Discord account. Please link it and try again.',
        });
      return;
    }

    const verified = session.user.name === hypixelDiscordId;
    if (!verified) {
      res.status(403).json({ error: 'IGN verification failed' });
      return;
    }

    const [userObj] = await db
      .select()
      .from(user)
      .where(eq(user.id, session.user.id));
    if (!userObj) {
      res.status(404).json({ error: 'User not found' });
      return;
    }
    userSelectSchema.parse(userObj);

    const [updatedUser] = await db
      .update(user)
      .set({ minecraftId })
      .where(eq(user.id, session.user.id))
      .returning();
    if (!updatedUser) {
      res.status(500).json({ error: 'Failed to update IGN' });
      return;
    }

    res.json({ message: 'IGN verified successfully', minecraftId });
  } catch (error) {
    res.status(400).json({
      error: error instanceof Error ? error.message : 'Unknown error',
    });
  }
});

export default router;
