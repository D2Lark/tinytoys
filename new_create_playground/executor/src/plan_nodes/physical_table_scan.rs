use crate::plan_nodes::PlanNode;

#[derive(Debug, Clone)]
pub struct PhysicalTableScan{}
impl PlanNode for PhysicalTableScan{}