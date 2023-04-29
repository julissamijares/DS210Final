#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_graph() {
        let graph = load_graph("twitter_combined.txt");
        assert_eq!(graph.num_vertices(), 81306);
        assert_eq!(graph.num_edges(), 1768149);
    }

    #[test]
    fn test_degree_distribution() {
        let graph = load_graph("twitter_combined.txt");
        let dist = degree_distribution(&graph);
        assert_eq!(dist[0], 0.0);
        assert_eq!(dist[1], 0.0);
        assert!(dist[2] > 0.0);
        assert!(dist[2] < dist[3]);
        assert!(dist[3] < dist[4]);
        assert!(dist[4] < dist[5]);
        assert!(dist[5] < dist[6]);
        assert!(dist[6] < dist[7]);
    }

    #[test]
    fn test_neighbor_distribution() {
        let graph = load_graph("twitter_combined.txt");
        let dist = neighbor_distribution(&graph, 2);
        assert_eq!(dist[0], 0.0);
        assert_eq!(dist[1], 0.0);
        assert!(dist[2] > 0.0);
        assert!(dist[2] < dist[3]);
        assert!(dist[3] < dist[4]);
        assert!(dist[4] < dist[5]);
        assert!(dist[5] < dist[6]);
        assert!(dist[6] < dist[7]);
    }
}
