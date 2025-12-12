import { seedContextProperty } from "../repository/initial-data/seed-context-property";

seedContextProperty()
  .then(() => console.log("Done."))
  .catch((err) => {
    console.error("Context/Property seed failed:", err);
    process.exit(1);
  });

