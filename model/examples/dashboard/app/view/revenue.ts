//@/ui/view/revenue.ts
import { Revenue } from '@/lib/data/schema/revenue'
import { RevenueForm } from '@/ui/graphics/form/revenue'
import { RevenueFormShape } from '@/ui/graphics/schema/revenue';
import { FormView } from './form'

export class RevenueView extends FormView<RevenueFormShape> {
    constructor(private readonly revenue?: Revenue) {
        super(new RevenueForm(revenue))
    }

    update(): void {
      // Implement the update method to update the revenue form
    }

    async handleSubmit(): Promise<void> {
      // Implement the handleSubmit method to handle form submission
    }

    handleCancel(): void {
      // Implement the handleCancel method to handle form cancellation
    }
  }
