'use client';

// Update this import
import { RevenueChartDisplay } from '@graphics/schema/revenue';
import { ContainerCard } from '@/graphics/card/card';
import {
  Bar,
  BarChart,
  CartesianGrid,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis
} from 'recharts';

export function RevenueChart({ data }: { data: RevenueChartDisplay }) {
  return (
    <ContainerCard>
      <h3 className="text-lg font-medium">Revenue Over Time</h3>

      <div className="mt-4 h-[300px]">
        <ResponsiveContainer width="100%" height="100%">
          <BarChart
            data={data.data}
            margin={{
              top: 5,
              right: 10,
              left: 10,
              bottom: 5,
            }}
          >
            <CartesianGrid strokeDasharray="3 3" vertical={false} />
            <XAxis
              dataKey="date"
              tickLine={false}
              axisLine={false}
            />
            <YAxis
              tickLine={false}
              axisLine={false}
              tickFormatter={(value) => `$${value}`}
            />
            <Tooltip
              formatter={(value) => [`$${value}`, 'Revenue']}
              labelFormatter={(label) => `Date: ${label}`}
            />
            <Bar
              dataKey="amount"
              fill="#0EA5E9"
              radius={[4, 4, 0, 0]}
              name="Revenue"
            />
          </BarChart>
        </ResponsiveContainer>
      </div>

      <div className="mt-4">
        <div className="flex items-center justify-between text-sm">
          <div>
            <p className="text-gray-500">Total Revenue</p>
            <p className="text-xl font-medium">${data.totalRevenue.toLocaleString()}</p>
          </div>
          {data.growthRate !== undefined && (
            <div>
              <p className="text-gray-500">Growth</p>
              <p className={`text-xl font-medium ${data.growthRate >= 0 ? 'text-green-500' : 'text-red-500'}`}>
                {data.growthRate >= 0 ? '+' : ''}{data.growthRate.toFixed(1)}%
              </p>
            </div>
          )}
        </div>
      </div>
    </ContainerCard>
  );
}
