"use server";

import { revalidatePath } from "next/cache";

type Users = {
  id: string;
  name: string;
  email: string;
};

type NewUser = {
  name: string;
  email: string;
};
export async function getAllUsers() {
  const response = await fetch("http://localhost:3000/users");
  if (!response.ok) {
    throw new Error("Kunde inte hämta data");
  }
  const data = await response.json();

  return data.users;
}

export async function addUser(data: NewUser) {
  try {
    const response = await fetch("http://localhost:3000/users", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(data),
    });
  } catch (error) {
    console.log("Error:", error);
  }
  revalidatePath("/users");
}
