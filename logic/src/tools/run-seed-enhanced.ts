import { seedEnhanced } from "../repository/initial-data/seed-enhanced";

seedEnhanced()
  .then(() => console.log("Done."))
  .catch((err) => {
    console.error("Enhanced seed failed:", err);
    process.exit(1);
  });

