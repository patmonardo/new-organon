//@view/revenue.ts
import { Revenue } from '@/data/schema/revenue'
import { RevenueForm } from '@graphics/form/revenue'
import { RevenueFormShape } from '@graphics/schema/revenue';
import { FormView } from './form'

export class RevenueView extends FormView<RevenueFormShape> {
    constructor(private readonly revenue?: Revenue) {
        super(new RevenueForm(revenue))
    }

  }

