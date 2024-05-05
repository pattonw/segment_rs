import funrustlib


def test_connected_components():
    nodes = [1, 2, 3, 4, 5, 6, 7, 8, 9, 123, 11251, 13613261]
    edges = [
        (1, 2),
        (3, 4),
        (4, 5),
        (5, 123),
        (7, 13613261),
    ]
    scores = [
        0.0,
        0.0,
        0.0,
        0.0,
        2.0,
    ]
    threshold = 1.0

    ccs_a = funrustlib.segment.connected_components(nodes, edges, None, None)
    ccs_b = funrustlib.segment.connected_components(nodes, edges, scores, threshold)

    assert len(set(ccs_a)) == 7
    assert len(set(ccs_b)) == 8
