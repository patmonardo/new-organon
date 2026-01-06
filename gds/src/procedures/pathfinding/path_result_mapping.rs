use crate::algo::common::result_builders::PathResult as CorePathResult;
use crate::procedures::traits::PathResult as ProcedurePathResult;

pub(crate) fn core_to_procedure_path_result(path: CorePathResult) -> ProcedurePathResult {
    ProcedurePathResult {
        source: path.source,
        target: path.target,
        path: path.path,
        cost: path.cost,
    }
}
