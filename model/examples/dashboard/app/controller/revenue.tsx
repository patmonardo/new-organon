import { RevenueChart } from "@/graphics/chart/revenue";
import { RevenueChartDisplay } from "@graphics/schema/revenue";
import { RevenueModel } from "@model/revenue";

export class RevenueController {
  /**
   * Renders the revenue chart component
   */
  static async displayChart() {
    // Fetch revenue data using the model
    const result = await RevenueModel.getMonthlyMetrics();
    if (result.status !== "success" || !result.data) {
      console.error("Error fetching revenue data:", result.message);
      return null;
    }
    const revenueData = result.data;
    // Transform to display format
    const chartData: RevenueChartDisplay = {
      interval: "month",
      data: revenueData.map((item) => ({
        date: formatMonthDisplay(item.month),
        amount: item.revenue,
      })),
      totalRevenue: revenueData.reduce((sum, item) => sum + item.revenue, 0),
      previousPeriodRevenue: calculatePreviousPeriod(revenueData),
      growthRate: calculateGrowthRate(revenueData),
    };

    // Render the chart component
    return <RevenueChart data={chartData} />;
  }
}

// Helper functions for calculations
function formatMonthDisplay(date: Date): string {
  return new Intl.DateTimeFormat("en-US", { month: "short" }).format(date);
}

function calculatePreviousPeriod(
  data: { month: Date; revenue: number }[]
): number {
  const midpoint = Math.floor(data.length / 2);
  const previousData = data.slice(0, midpoint);
  return previousData.reduce((sum, item) => sum + item.revenue, 0);
}

function calculateGrowthRate(data: { month: Date; revenue: number }[]): number {
  const midpoint = Math.floor(data.length / 2);
  const currentPeriod = data
    .slice(midpoint)
    .reduce((sum, item) => sum + item.revenue, 0);
  const previousPeriod = data
    .slice(0, midpoint)
    .reduce((sum, item) => sum + item.revenue, 0);

  if (previousPeriod === 0) return 0;
  return ((currentPeriod - previousPeriod) / previousPeriod) * 100;
}
