import { seedForms } from "../repository/initial-data/seed-forms";

seedForms()
  .then(() => console.log("Done."))
  .catch((err) => {
    console.error("Seed failed:", err);
    process.exit(1);
  });
