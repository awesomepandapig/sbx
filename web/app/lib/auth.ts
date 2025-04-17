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

export async function signIn(callbackURL?: string) {
  if (!callbackURL) {
    callbackURL = `${DOMAIN}/trade/JSP`;
  }

  try {
    const response = await fetch(`${API_URL}/auth/sign-in/social`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        provider: "discord",
        newUserCallbackURL: `${DOMAIN}/verify-ign`,
        callbackURL,
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
    window.location.href = DOMAIN;

    if (!response.ok) {
      throw new Error("something went wrong"); // TODO: error message
    }
  } catch (error) {
    console.log(error); //TODO: handle error
  }
}

export async function requireUserSession({ request }: LoaderFunctionArgs) {
  try {
    const cookie = request.headers.get("Cookie");
    const session = await getSession(cookie);
    if (!session) {
      throw redirect("/", 302);
    }
    return session.get("user");
  } catch (error) {
    throw redirect("/", 302);
  }
}

export async function getUserSession({ request }: LoaderFunctionArgs) {
  try {
    const cookie = request.headers.get("Cookie");
    const session = await getSession(cookie);
    const user = session.user;
    return user;
  } catch (error) {
    return null;
  }
}
