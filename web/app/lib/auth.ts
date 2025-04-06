import { redirect } from "@remix-run/react";
import { API_URL, DOMAIN } from "~/lib/config";
import { LoaderFunctionArgs } from "@remix-run/node";

export async function getSession(cookie: string | null) {
  try {
    const response = await fetch(`${API_URL}/auth/get-session`, {
      headers: {
        Cookie: cookie || "",
      },
    });
    if (!response.ok) {
      throw new Error(""); // TODO: error message
    }

    // If the session response is empty user is not logged-in
    const responseText = await response.text();
    if (responseText.length == 0) {
      // && responseText != "null"
      return;
    }

    const data = JSON.parse(responseText);
    return data;
  } catch (error) {
    console.log(error); // TODO: handle error
    return;
  }
}

export async function requireUserSession(request: Request) {
  const cookie = request.headers.get("Cookie");
  const session = await getSession(cookie);
  if (!session || !session.user) {
    throw redirect("/", 302);
  }
  return session.user;
}

export async function signIn() {
  try {
    const response = await fetch(`${API_URL}/auth/sign-in/social`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        provider: "discord",
        newUserCallbackURL: "/verify",
        callbackURL: `${DOMAIN}/trade/FRY`, // TODO: CHANGE CALLBACK URL TO URL PARAM ie /trade/JSP etc.
      }),
    });
    if (!response.ok) {
      throw new Error("something went wrong"); // TODO: error message
    }

    const data = await response.json();
    if (data.url) {
      window.location.href = data.url;
    }

    console.log(data);
  } catch (error) {
    console.log(error); //TODO: handle error
  }
}

export async function signOut() {
  try {
    const response = await fetch(`${API_URL}/auth/sign-out`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      credentials: "include",
      body: JSON.stringify({}),
    });
    if (!response.ok) {
      throw new Error("something went wrong"); // TODO: error message
    }
    window.location.href = `${DOMAIN}`;
  } catch (error) {
    console.log(error); //TODO: handle error
  }
}

export async function authLoader({ request }: LoaderFunctionArgs) {
  try {
    const cookie = request.headers.get("Cookie");
    const session = await getSession(cookie);

    if (session && session.user) {
      return { user: session.user };
    }

    return { user: null };
  } catch (error) {
    console.error("Error in loader:", error);
    return { user: null };
  }
}
