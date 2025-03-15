import { betterAuth } from 'better-auth';
import { jwt, bearer, openAPI } from 'better-auth/plugins';
import { drizzleAdapter } from 'better-auth/adapters/drizzle';
import { db } from '@db';
import * as schema from '@db';
import { DOMAIN, PROD } from '@config';

export const auth = betterAuth({
  database: drizzleAdapter(db, {
    provider: 'pg',
    schema,
  }),
  user: {
    additionalFields: {
      minecraftId: {
        type: 'string',
        required: false,
        defaultValue: null,
      },
    },
  },
  socialProviders: {
    discord: {
      clientId: process.env.DISCORD_CLIENT_ID as string,
      clientSecret: process.env.DISCORD_CLIENT_SECRET as string,
    },
  },
  plugins: [openAPI(), jwt(), bearer()],
  advanced: {
    crossSubDomainCookies: {
      enabled: PROD,
      domain: `.${DOMAIN}`, // Domain with a leading period
    },
    defaultCookieAttributes: {
      secure: PROD, // Only secure in production
      httpOnly: true,
      sameSite: PROD ? 'none' : 'lax', // Allows CORS-based cookie sharing across subdomains
      partitioned: PROD, // Only partitioned when secure
    },
  },
  trustedOrigins: PROD
    ? [`https://${DOMAIN}`, `https://api.${DOMAIN}`]
    : ['http://localhost'],
});
