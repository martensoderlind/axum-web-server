"use server";

type Users = {
  id: string;
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
