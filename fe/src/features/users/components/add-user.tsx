"use client";
import React, { useState, ChangeEvent, FormEvent } from "react";
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from "../../../../components/ui/card";
import { Label } from "../../../../components/ui/label";
import { Input } from "../../../../components/ui/input";
import { Button } from "../../../../components/ui/button";
import { addUser } from "../actions";

type FormData = {
  name: string;
  email: string;
};

type StatusType = "idle" | "loading" | "success" | "error";

export default function AddUser() {
  const [formData, setFormData] = useState<FormData>({
    name: "",
    email: "",
  });
  const [status, setStatus] = useState<StatusType>("idle");

  const handleSubmit = async (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setStatus("loading");
    await addUser(formData);
    setStatus("success");
    setFormData({ name: "", email: "" });
  };

  const handleChange = (e: ChangeEvent<HTMLInputElement>) => {
    setFormData({
      ...formData,
      [e.target.name]: e.target.value,
    });
  };

  return (
    <Card className="w-full max-w-md mx-auto">
      <CardHeader>
        <CardTitle>Registrera användare</CardTitle>
      </CardHeader>
      <CardContent>
        <form onSubmit={handleSubmit} className="space-y-4">
          <div className="space-y-2">
            <Label htmlFor="name">Namn</Label>
            <Input
              id="name"
              name="name"
              type="text"
              value={formData.name}
              onChange={handleChange}
              required
              className="w-full bg-gray-100"
            />
          </div>

          <div className="space-y-2">
            <Label htmlFor="email">Email</Label>
            <Input
              id="email"
              name="email"
              type="email"
              value={formData.email}
              onChange={handleChange}
              required
              className="w-full bg-gray-100"
            />
          </div>

          <Button
            type="submit"
            className="w-full"
            disabled={status === "loading"}
          >
            {status === "loading" ? "Skickar..." : "Skicka"}
          </Button>

          {status === "success" && (
            <p className="text-green-600 text-center">Användare skapad!</p>
          )}
          {status === "error" && (
            <p className="text-red-600 text-center">
              Något gick fel, försök igen.
            </p>
          )}
        </form>
      </CardContent>
    </Card>
  );
}
