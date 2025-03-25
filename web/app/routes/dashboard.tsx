import type { LoaderFunctionArgs } from "@remix-run/node";
import { requireUserSession } from "~/lib/auth";
import { redirect } from "@remix-run/node";

export const loader = async ({ request }: LoaderFunctionArgs) => {
  const user = await requireUserSession(request);
  // if (!user.minecraftId) {
  //   throw redirect("/verify");
  // } 
  // TODO: Validate user has a session, if user doos not have a session disable interactivity on order book
  // TODO: show avatar if user is logged in
  return null;
};

export default function Dashboard() {
  return (
    <div>
      <h1>Welcome to your Dashboard!</h1>
    </div>
  );
}
