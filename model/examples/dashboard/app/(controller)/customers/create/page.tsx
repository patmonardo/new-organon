import { CustomerController } from "@/(controller)/inner/customer";

export default async function Page() {
  return (
    <main className="max-w-4xl mx-auto p-4">
      {await CustomerController.createForm()}
    </main>
  );
}
