import { redirect } from "@remix-run/react";
import { API_URL } from "~/lib/config";

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

    const data = await response.json();
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
        callbackURL: "http://localhost:5173/dashboard", // TODO: change to domain
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
