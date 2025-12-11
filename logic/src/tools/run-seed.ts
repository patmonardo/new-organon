import { seedForms } from "../initial-data/seed-forms";

seedForms()
  .then(() => console.log("Done."))
  .catch((err) => {
    console.error("Seed failed:", err);
    process.exit(1);
  });
