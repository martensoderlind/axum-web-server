import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import AddUser from "./add-user";
import Users from "./users";

export function UserTabs() {
  return (
    <Tabs
      defaultValue="users"
      className="w-[400px] my-6 container mx-auto bg-white p-4 rounded-md"
    >
      <TabsList className="grid w-full grid-cols-2">
        <TabsTrigger value="users">Users</TabsTrigger>
        <TabsTrigger value="add">Add user</TabsTrigger>
      </TabsList>
      <TabsContent value="users">
        <Users />
      </TabsContent>
      <TabsContent value="add">
        <AddUser />
      </TabsContent>
    </Tabs>
  );
}
