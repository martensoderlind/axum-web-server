import { User as UserType } from "../types";

type Props = {
  user: UserType;
};

export function User({ user }: Props) {
  return (
    <div className="flex m-2">
      <p className="px-2">{user.name}</p>
      <p className="px-2">{user.email}</p>
    </div>
  );
}
