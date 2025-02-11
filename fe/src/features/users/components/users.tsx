import { getAllUsers } from "../actions";
import { User as UserType } from "../types";
import { User } from "./user";

export default async function Users() {
  const users: UserType[] = await getAllUsers();
  console.log("users", users);

  return (
    <div>
      <p className="m-2">users</p>
      {users.map((user, index) => (
        <User user={user} key={index} />
      ))}
    </div>
  );
}
