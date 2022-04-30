use std::{cmp::Ordering, collections::BinaryHeap};

use bevy::utils::HashMap;

use crate::*;

use super::is_connected;

pub struct AStar<'a, T: GridTile> {
    grid: &'a Grid<T>,
}

impl<'a, T: GridTile> AStar<'a, T> {
    pub fn new(grid: &'a Grid<T>) -> Self {
        Self { grid }
    }

    pub fn search(
        &self,
        start: T::Cell,
        goal: T::Cell,
        edge_weight: EdgeWeight,
    ) -> Option<Vec<T::Cell>> {
        let mut heap: BinaryHeap<AStarNode<T::Cell>> = BinaryHeap::default();
        let mut path: HashMap<T::Cell, T::Cell> = HashMap::default();
        let mut cost: HashMap<T::Cell, usize> = HashMap::default();

        heap.push(AStarNode::new(start, 0));
        path.insert(start, start);
        cost.insert(start, 0);

        while let Some(node) = heap.pop() {
            if node.cell == goal {
                let mut waypoints: Vec<T::Cell> = Vec::new();
                let mut next = node.cell;
                while next != start {
                    waypoints.push(next);
                    next = path[&next];
                }
                waypoints.push(start);
                waypoints.reverse();
                return Some(waypoints);
            }

            let tile = self.grid.get_tile(node.cell);
            for neighbor_cell in tile.neighbors(node.cell) {
                if let Some(neighbor) = self.grid.try_get_tile(neighbor_cell) {
                    if !neighbor.is_walkable() {
                        continue;
                    }

                    if !is_connected(node.cell, neighbor, neighbor_cell) {
                        continue;
                    }

                    let edge_cost = edge_weight.cost(tile, node.cell, neighbor_cell, self.grid);
                    let accumulated_cost = cost[&node.cell] + edge_cost;

                    if !cost.contains_key(&neighbor_cell) || accumulated_cost < cost[&neighbor_cell]
                    {
                        path.insert(neighbor_cell, node.cell);
                        cost.insert(neighbor_cell, accumulated_cost);
                        let heuristic = neighbor.heuristic(neighbor_cell, goal);
                        heap.push(AStarNode::new(neighbor_cell, accumulated_cost + heuristic));
                    }
                }
            }
        }

        None
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct AStarNode<C: GridCell> {
    cell: C,
    cost: usize,
}

impl<C: GridCell> AStarNode<C> {
    fn new(cell: C, cost: usize) -> Self {
        Self { cell, cost }
    }
}

impl<C: GridCell> Ord for AStarNode<C> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl<C: GridCell> PartialOrd for AStarNode<C> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
