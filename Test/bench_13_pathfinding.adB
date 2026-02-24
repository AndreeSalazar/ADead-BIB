// ============================================================
// ADead-BIB Benchmark Suite — 13: Pathfinding & Graphs
// ============================================================
// Graph algorithms on small fixed grids/graphs using
// scalar variables. Combines procedural and OOP-light.
//
// Compilar: adeadc bench_13_pathfinding.adB -o bench13.exe
// ============================================================

int mod_int(int a, int b) {
    return a - (a / b) * b
}

int abs_val(int x) {
    if x < 0 {
        return 0 - x
    }
    return x
}

int min_val(int a, int b) {
    if a < b {
        return a
    }
    return b
}

// ============================================================
// TEST 1: Manhattan distance (procedural, 10M pairs)
// ============================================================

void bench_manhattan() {
    printf("--- Test 1: Manhattan Distance (10M pairs) ---\n")

    int total = 0
    int seed = 17
    int i = 0

    while i < 10000000 {
        seed = seed * 1103515245 + 12345
        int s = seed
        if s < 0 { s = 0 - s }
        int x1 = mod_int(s, 1000)
        seed = seed * 1103515245 + 12345
        s = seed
        if s < 0 { s = 0 - s }
        int y1 = mod_int(s, 1000)
        seed = seed * 1103515245 + 12345
        s = seed
        if s < 0 { s = 0 - s }
        int x2 = mod_int(s, 1000)
        seed = seed * 1103515245 + 12345
        s = seed
        if s < 0 { s = 0 - s }
        int y2 = mod_int(s, 1000)

        total = total + abs_val(x2 - x1) + abs_val(y2 - y1)
        if total > 1000000000 {
            total = mod_int(total, 1000000)
        }
        i = i + 1
    }

    printf("  total distance = ")
    printf(total)
    printf("\n\n")
}

// ============================================================
// TEST 2: Flood fill on 4x4 grid (procedural, 10K grids)
// ============================================================

void bench_flood_fill() {
    printf("--- Test 2: Flood Fill 4x4 Grid (10K iterations) ---\n")

    int total_filled = 0
    int iter = 0
    int seed = 42

    while iter < 10000 {
        // Generate 4x4 grid: 0 = empty, 1 = wall
        // ~30% walls
        seed = seed * 1103515245 + 12345
        int s = seed  if s < 0 { s = 0 - s }
        int g00 = 0  // start always empty
        seed = seed * 1103515245 + 12345  s = seed  if s < 0 { s = 0 - s }
        int g01 = 0  if mod_int(s, 10) < 3 { g01 = 1 }
        seed = seed * 1103515245 + 12345  s = seed  if s < 0 { s = 0 - s }
        int g02 = 0  if mod_int(s, 10) < 3 { g02 = 1 }
        seed = seed * 1103515245 + 12345  s = seed  if s < 0 { s = 0 - s }
        int g03 = 0  if mod_int(s, 10) < 3 { g03 = 1 }
        seed = seed * 1103515245 + 12345  s = seed  if s < 0 { s = 0 - s }
        int g10 = 0  if mod_int(s, 10) < 3 { g10 = 1 }
        seed = seed * 1103515245 + 12345  s = seed  if s < 0 { s = 0 - s }
        int g11 = 0  if mod_int(s, 10) < 3 { g11 = 1 }
        seed = seed * 1103515245 + 12345  s = seed  if s < 0 { s = 0 - s }
        int g12 = 0  if mod_int(s, 10) < 3 { g12 = 1 }
        seed = seed * 1103515245 + 12345  s = seed  if s < 0 { s = 0 - s }
        int g13 = 0  if mod_int(s, 10) < 3 { g13 = 1 }
        seed = seed * 1103515245 + 12345  s = seed  if s < 0 { s = 0 - s }
        int g20 = 0  if mod_int(s, 10) < 3 { g20 = 1 }
        seed = seed * 1103515245 + 12345  s = seed  if s < 0 { s = 0 - s }
        int g21 = 0  if mod_int(s, 10) < 3 { g21 = 1 }
        seed = seed * 1103515245 + 12345  s = seed  if s < 0 { s = 0 - s }
        int g22 = 0  if mod_int(s, 10) < 3 { g22 = 1 }
        seed = seed * 1103515245 + 12345  s = seed  if s < 0 { s = 0 - s }
        int g23 = 0  if mod_int(s, 10) < 3 { g23 = 1 }
        seed = seed * 1103515245 + 12345  s = seed  if s < 0 { s = 0 - s }
        int g30 = 0  if mod_int(s, 10) < 3 { g30 = 1 }
        seed = seed * 1103515245 + 12345  s = seed  if s < 0 { s = 0 - s }
        int g31 = 0  if mod_int(s, 10) < 3 { g31 = 1 }
        seed = seed * 1103515245 + 12345  s = seed  if s < 0 { s = 0 - s }
        int g32 = 0  if mod_int(s, 10) < 3 { g32 = 1 }
        seed = seed * 1103515245 + 12345  s = seed  if s < 0 { s = 0 - s }
        int g33 = 0  if mod_int(s, 10) < 3 { g33 = 1 }

        // Flood fill from (0,0): mark with 2
        g00 = 2
        // Propagate (iterative, multiple passes)
        int changed = 1
        int pass = 0
        while changed == 1 {
            if pass > 8 { changed = 0 }
            if changed == 1 {
                changed = 0
                // Check each cell: if neighbor is 2 and cell is 0, fill
                if g01 == 0 { if g00 == 2 { g01 = 2  changed = 1 } }
                if g10 == 0 { if g00 == 2 { g10 = 2  changed = 1 } }
                if g00 == 0 { if g01 == 2 { g00 = 2  changed = 1 } }
                if g02 == 0 { if g01 == 2 { g02 = 2  changed = 1 } }
                if g11 == 0 { if g01 == 2 { g11 = 2  changed = 1 } }
                if g01 == 0 { if g02 == 2 { g01 = 2  changed = 1 } }
                if g03 == 0 { if g02 == 2 { g03 = 2  changed = 1 } }
                if g12 == 0 { if g02 == 2 { g12 = 2  changed = 1 } }
                if g02 == 0 { if g03 == 2 { g02 = 2  changed = 1 } }
                if g13 == 0 { if g03 == 2 { g13 = 2  changed = 1 } }
                if g00 == 0 { if g10 == 2 { g00 = 2  changed = 1 } }
                if g11 == 0 { if g10 == 2 { g11 = 2  changed = 1 } }
                if g20 == 0 { if g10 == 2 { g20 = 2  changed = 1 } }
                if g10 == 0 { if g11 == 2 { g10 = 2  changed = 1 } }
                if g12 == 0 { if g11 == 2 { g12 = 2  changed = 1 } }
                if g21 == 0 { if g11 == 2 { g21 = 2  changed = 1 } }
                if g11 == 0 { if g12 == 2 { g11 = 2  changed = 1 } }
                if g13 == 0 { if g12 == 2 { g13 = 2  changed = 1 } }
                if g22 == 0 { if g12 == 2 { g22 = 2  changed = 1 } }
                if g12 == 0 { if g13 == 2 { g12 = 2  changed = 1 } }
                if g23 == 0 { if g13 == 2 { g23 = 2  changed = 1 } }
                if g10 == 0 { if g20 == 2 { g10 = 2  changed = 1 } }
                if g21 == 0 { if g20 == 2 { g21 = 2  changed = 1 } }
                if g30 == 0 { if g20 == 2 { g30 = 2  changed = 1 } }
                if g20 == 0 { if g21 == 2 { g20 = 2  changed = 1 } }
                if g22 == 0 { if g21 == 2 { g22 = 2  changed = 1 } }
                if g31 == 0 { if g21 == 2 { g31 = 2  changed = 1 } }
                if g21 == 0 { if g22 == 2 { g21 = 2  changed = 1 } }
                if g23 == 0 { if g22 == 2 { g23 = 2  changed = 1 } }
                if g32 == 0 { if g22 == 2 { g32 = 2  changed = 1 } }
                if g22 == 0 { if g23 == 2 { g22 = 2  changed = 1 } }
                if g33 == 0 { if g23 == 2 { g33 = 2  changed = 1 } }
                if g30 == 0 { if g31 == 2 { g30 = 2  changed = 1 } }
                if g31 == 0 { if g30 == 2 { g31 = 2  changed = 1 } }
                if g32 == 0 { if g31 == 2 { g32 = 2  changed = 1 } }
                if g31 == 0 { if g32 == 2 { g31 = 2  changed = 1 } }
                if g33 == 0 { if g32 == 2 { g33 = 2  changed = 1 } }
                if g32 == 0 { if g33 == 2 { g32 = 2  changed = 1 } }
                pass = pass + 1
            }
        }

        // Count filled cells
        int filled = 0
        if g00 == 2 { filled = filled + 1 }
        if g01 == 2 { filled = filled + 1 }
        if g02 == 2 { filled = filled + 1 }
        if g03 == 2 { filled = filled + 1 }
        if g10 == 2 { filled = filled + 1 }
        if g11 == 2 { filled = filled + 1 }
        if g12 == 2 { filled = filled + 1 }
        if g13 == 2 { filled = filled + 1 }
        if g20 == 2 { filled = filled + 1 }
        if g21 == 2 { filled = filled + 1 }
        if g22 == 2 { filled = filled + 1 }
        if g23 == 2 { filled = filled + 1 }
        if g30 == 2 { filled = filled + 1 }
        if g31 == 2 { filled = filled + 1 }
        if g32 == 2 { filled = filled + 1 }
        if g33 == 2 { filled = filled + 1 }

        total_filled = total_filled + filled
        iter = iter + 1
    }

    printf("  total filled cells = ")
    printf(total_filled)
    printf("\n\n")
}

// ============================================================
// TEST 3: Dijkstra on 4-node graph (OOP-light, 100K)
// ============================================================

void bench_dijkstra() {
    printf("--- Test 3: Dijkstra 4-Node Graph (100K iterations) ---\n")

    int total_cost = 0
    int iter = 0
    int seed = 99

    while iter < 100000 {
        // Generate edge weights (complete graph: 6 edges)
        seed = seed * 1103515245 + 12345
        int s = seed  if s < 0 { s = 0 - s }
        int w01 = mod_int(s, 100) + 1
        seed = seed * 1103515245 + 12345  s = seed  if s < 0 { s = 0 - s }
        int w02 = mod_int(s, 100) + 1
        seed = seed * 1103515245 + 12345  s = seed  if s < 0 { s = 0 - s }
        int w03 = mod_int(s, 100) + 1
        seed = seed * 1103515245 + 12345  s = seed  if s < 0 { s = 0 - s }
        int w12 = mod_int(s, 100) + 1
        seed = seed * 1103515245 + 12345  s = seed  if s < 0 { s = 0 - s }
        int w13 = mod_int(s, 100) + 1
        seed = seed * 1103515245 + 12345  s = seed  if s < 0 { s = 0 - s }
        int w23 = mod_int(s, 100) + 1

        // Dijkstra from node 0
        int d0 = 0  int d1 = 999999  int d2 = 999999  int d3 = 999999
        int v0 = 0  int v1 = 0  int v2 = 0  int v3 = 0

        // Iteration 1: process node 0 (smallest unvisited)
        v0 = 1
        if w01 < d1 { d1 = w01 }
        if w02 < d2 { d2 = w02 }
        if w03 < d3 { d3 = w03 }

        // Find next smallest unvisited
        int next = 999999  int next_id = -1
        if v1 == 0 { if d1 < next { next = d1  next_id = 1 } }
        if v2 == 0 { if d2 < next { next = d2  next_id = 2 } }
        if v3 == 0 { if d3 < next { next = d3  next_id = 3 } }

        // Iteration 2: process next node
        if next_id == 1 {
            v1 = 1
            int nd2 = d1 + w12  if nd2 < d2 { d2 = nd2 }
            int nd3 = d1 + w13  if nd3 < d3 { d3 = nd3 }
        }
        if next_id == 2 {
            v2 = 1
            int nd1 = d2 + w12  if nd1 < d1 { d1 = nd1 }
            int nd3 = d2 + w23  if nd3 < d3 { d3 = nd3 }
        }
        if next_id == 3 {
            v3 = 1
            int nd1 = d3 + w13  if nd1 < d1 { d1 = nd1 }
            int nd2 = d3 + w23  if nd2 < d2 { d2 = nd2 }
        }

        // Iteration 3: next unvisited
        next = 999999  next_id = -1
        if v1 == 0 { if d1 < next { next = d1  next_id = 1 } }
        if v2 == 0 { if d2 < next { next = d2  next_id = 2 } }
        if v3 == 0 { if d3 < next { next = d3  next_id = 3 } }

        if next_id == 1 {
            v1 = 1
            int nd2 = d1 + w12  if nd2 < d2 { d2 = nd2 }
            int nd3 = d1 + w13  if nd3 < d3 { d3 = nd3 }
        }
        if next_id == 2 {
            v2 = 1
            int nd1 = d2 + w12  if nd1 < d1 { d1 = nd1 }
            int nd3 = d2 + w23  if nd3 < d3 { d3 = nd3 }
        }
        if next_id == 3 {
            v3 = 1
            int nd1 = d3 + w13  if nd1 < d1 { d1 = nd1 }
            int nd2 = d3 + w23  if nd2 < d2 { d2 = nd2 }
        }

        total_cost = total_cost + d3
        if total_cost > 1000000000 {
            total_cost = mod_int(total_cost, 1000000)
        }
        iter = iter + 1
    }

    printf("  total shortest path cost = ")
    printf(total_cost)
    printf("\n\n")
}

// ============================================================
// TEST 4: A* heuristic computation (procedural, 10M)
// ============================================================

void bench_astar_heuristic() {
    printf("--- Test 4: A* Heuristic f=g+h (10M positions) ---\n")

    int total_f = 0
    int goal_x = 99
    int goal_y = 99
    int seed = 7
    int i = 0

    while i < 10000000 {
        seed = seed * 1103515245 + 12345
        int s = seed  if s < 0 { s = 0 - s }
        int x = mod_int(s, 100)
        seed = seed * 1103515245 + 12345
        s = seed  if s < 0 { s = 0 - s }
        int y = mod_int(s, 100)

        // g = steps from origin (manhattan to 0,0)
        int g = x + y
        // h = manhattan distance to goal
        int h = abs_val(goal_x - x) + abs_val(goal_y - y)
        // f = g + h
        int f = g + h

        total_f = total_f + f
        if total_f > 1000000000 {
            total_f = mod_int(total_f, 1000000)
        }
        i = i + 1
    }

    printf("  total f values = ")
    printf(total_f)
    printf("\n\n")
}

// ============================================================
// TEST 5: Graph BFS on 4-node adjacency matrix (100K)
// ============================================================

void bench_bfs() {
    printf("--- Test 5: BFS 4-Node Graph (100K iterations) ---\n")

    int total_reachable = 0
    int iter = 0
    int seed = 31

    while iter < 100000 {
        // Generate random adjacency matrix (4x4, ~50% edges)
        seed = seed * 1103515245 + 12345  int s = seed  if s < 0 { s = 0 - s }
        int a01 = 0  if mod_int(s, 2) == 1 { a01 = 1 }
        seed = seed * 1103515245 + 12345  s = seed  if s < 0 { s = 0 - s }
        int a02 = 0  if mod_int(s, 2) == 1 { a02 = 1 }
        seed = seed * 1103515245 + 12345  s = seed  if s < 0 { s = 0 - s }
        int a03 = 0  if mod_int(s, 2) == 1 { a03 = 1 }
        seed = seed * 1103515245 + 12345  s = seed  if s < 0 { s = 0 - s }
        int a12 = 0  if mod_int(s, 2) == 1 { a12 = 1 }
        seed = seed * 1103515245 + 12345  s = seed  if s < 0 { s = 0 - s }
        int a13 = 0  if mod_int(s, 2) == 1 { a13 = 1 }
        seed = seed * 1103515245 + 12345  s = seed  if s < 0 { s = 0 - s }
        int a23 = 0  if mod_int(s, 2) == 1 { a23 = 1 }

        // BFS from node 0
        int r0 = 1  int r1 = 0  int r2 = 0  int r3 = 0

        // Level 1: direct from 0
        if a01 == 1 { r1 = 1 }
        if a02 == 1 { r2 = 1 }
        if a03 == 1 { r3 = 1 }

        // Level 2: from newly reachable nodes
        if r1 == 1 {
            if a12 == 1 { r2 = 1 }
            if a13 == 1 { r3 = 1 }
        }
        if r2 == 1 {
            if a23 == 1 { r3 = 1 }
        }

        int reachable = r0 + r1 + r2 + r3
        total_reachable = total_reachable + reachable
        iter = iter + 1
    }

    printf("  total reachable nodes = ")
    printf(total_reachable)
    printf("\n\n")
}

// ============================================================
// TEST 6: TSP brute force for 5 cities (procedural, 1K configs)
// ============================================================

int tsp_route_cost(int d01, int d02, int d03, int d04,
                   int d12, int d13, int d14,
                   int d23, int d24, int d34,
                   int c0, int c1, int c2, int c3, int c4) {
    // Compute cost of route c0->c1->c2->c3->c4->c0
    int cost = 0
    // Segment c0->c1
    int a = c0  int b = c1
    if a > b { int t = a  a = b  b = t }
    if a == 0 {
        if b == 1 { cost = cost + d01 }
        if b == 2 { cost = cost + d02 }
        if b == 3 { cost = cost + d03 }
        if b == 4 { cost = cost + d04 }
    }
    if a == 1 {
        if b == 2 { cost = cost + d12 }
        if b == 3 { cost = cost + d13 }
        if b == 4 { cost = cost + d14 }
    }
    if a == 2 {
        if b == 3 { cost = cost + d23 }
        if b == 4 { cost = cost + d24 }
    }
    if a == 3 {
        if b == 4 { cost = cost + d34 }
    }

    // Segment c1->c2
    a = c1  b = c2
    if a > b { int t = a  a = b  b = t }
    if a == 0 {
        if b == 1 { cost = cost + d01 }
        if b == 2 { cost = cost + d02 }
        if b == 3 { cost = cost + d03 }
        if b == 4 { cost = cost + d04 }
    }
    if a == 1 {
        if b == 2 { cost = cost + d12 }
        if b == 3 { cost = cost + d13 }
        if b == 4 { cost = cost + d14 }
    }
    if a == 2 {
        if b == 3 { cost = cost + d23 }
        if b == 4 { cost = cost + d24 }
    }
    if a == 3 {
        if b == 4 { cost = cost + d34 }
    }

    // Segment c2->c3
    a = c2  b = c3
    if a > b { int t = a  a = b  b = t }
    if a == 0 {
        if b == 1 { cost = cost + d01 }
        if b == 2 { cost = cost + d02 }
        if b == 3 { cost = cost + d03 }
        if b == 4 { cost = cost + d04 }
    }
    if a == 1 {
        if b == 2 { cost = cost + d12 }
        if b == 3 { cost = cost + d13 }
        if b == 4 { cost = cost + d14 }
    }
    if a == 2 {
        if b == 3 { cost = cost + d23 }
        if b == 4 { cost = cost + d24 }
    }
    if a == 3 {
        if b == 4 { cost = cost + d34 }
    }

    // Segment c3->c4
    a = c3  b = c4
    if a > b { int t = a  a = b  b = t }
    if a == 0 {
        if b == 1 { cost = cost + d01 }
        if b == 2 { cost = cost + d02 }
        if b == 3 { cost = cost + d03 }
        if b == 4 { cost = cost + d04 }
    }
    if a == 1 {
        if b == 2 { cost = cost + d12 }
        if b == 3 { cost = cost + d13 }
        if b == 4 { cost = cost + d14 }
    }
    if a == 2 {
        if b == 3 { cost = cost + d23 }
        if b == 4 { cost = cost + d24 }
    }
    if a == 3 {
        if b == 4 { cost = cost + d34 }
    }

    // Segment c4->c0 (return)
    a = c4  b = c0
    if a > b { int t = a  a = b  b = t }
    if a == 0 {
        if b == 1 { cost = cost + d01 }
        if b == 2 { cost = cost + d02 }
        if b == 3 { cost = cost + d03 }
        if b == 4 { cost = cost + d04 }
    }
    if a == 1 {
        if b == 2 { cost = cost + d12 }
        if b == 3 { cost = cost + d13 }
        if b == 4 { cost = cost + d14 }
    }
    if a == 2 {
        if b == 3 { cost = cost + d23 }
        if b == 4 { cost = cost + d24 }
    }
    if a == 3 {
        if b == 4 { cost = cost + d34 }
    }

    return cost
}

void bench_tsp() {
    printf("--- Test 6: TSP Brute Force 5 Cities (1K configs) ---\n")

    int total_min = 0
    int iter = 0
    int seed = 137

    while iter < 1000 {
        // Generate 10 distances
        seed = seed * 1103515245 + 12345  int s = seed  if s < 0 { s = 0 - s }
        int d01 = mod_int(s, 90) + 10
        seed = seed * 1103515245 + 12345  s = seed  if s < 0 { s = 0 - s }
        int d02 = mod_int(s, 90) + 10
        seed = seed * 1103515245 + 12345  s = seed  if s < 0 { s = 0 - s }
        int d03 = mod_int(s, 90) + 10
        seed = seed * 1103515245 + 12345  s = seed  if s < 0 { s = 0 - s }
        int d04 = mod_int(s, 90) + 10
        seed = seed * 1103515245 + 12345  s = seed  if s < 0 { s = 0 - s }
        int d12 = mod_int(s, 90) + 10
        seed = seed * 1103515245 + 12345  s = seed  if s < 0 { s = 0 - s }
        int d13 = mod_int(s, 90) + 10
        seed = seed * 1103515245 + 12345  s = seed  if s < 0 { s = 0 - s }
        int d14 = mod_int(s, 90) + 10
        seed = seed * 1103515245 + 12345  s = seed  if s < 0 { s = 0 - s }
        int d23 = mod_int(s, 90) + 10
        seed = seed * 1103515245 + 12345  s = seed  if s < 0 { s = 0 - s }
        int d24 = mod_int(s, 90) + 10
        seed = seed * 1103515245 + 12345  s = seed  if s < 0 { s = 0 - s }
        int d34 = mod_int(s, 90) + 10

        // Try all 24 permutations of cities 1-4 (city 0 fixed as start)
        // Permutations of {1,2,3,4}
        int best = 999999
        int c = 0

        c = tsp_route_cost(d01,d02,d03,d04,d12,d13,d14,d23,d24,d34, 0,1,2,3,4)
        if c < best { best = c }
        c = tsp_route_cost(d01,d02,d03,d04,d12,d13,d14,d23,d24,d34, 0,1,2,4,3)
        if c < best { best = c }
        c = tsp_route_cost(d01,d02,d03,d04,d12,d13,d14,d23,d24,d34, 0,1,3,2,4)
        if c < best { best = c }
        c = tsp_route_cost(d01,d02,d03,d04,d12,d13,d14,d23,d24,d34, 0,1,3,4,2)
        if c < best { best = c }
        c = tsp_route_cost(d01,d02,d03,d04,d12,d13,d14,d23,d24,d34, 0,1,4,2,3)
        if c < best { best = c }
        c = tsp_route_cost(d01,d02,d03,d04,d12,d13,d14,d23,d24,d34, 0,1,4,3,2)
        if c < best { best = c }
        c = tsp_route_cost(d01,d02,d03,d04,d12,d13,d14,d23,d24,d34, 0,2,1,3,4)
        if c < best { best = c }
        c = tsp_route_cost(d01,d02,d03,d04,d12,d13,d14,d23,d24,d34, 0,2,1,4,3)
        if c < best { best = c }
        c = tsp_route_cost(d01,d02,d03,d04,d12,d13,d14,d23,d24,d34, 0,2,3,1,4)
        if c < best { best = c }
        c = tsp_route_cost(d01,d02,d03,d04,d12,d13,d14,d23,d24,d34, 0,2,3,4,1)
        if c < best { best = c }
        c = tsp_route_cost(d01,d02,d03,d04,d12,d13,d14,d23,d24,d34, 0,2,4,1,3)
        if c < best { best = c }
        c = tsp_route_cost(d01,d02,d03,d04,d12,d13,d14,d23,d24,d34, 0,2,4,3,1)
        if c < best { best = c }
        c = tsp_route_cost(d01,d02,d03,d04,d12,d13,d14,d23,d24,d34, 0,3,1,2,4)
        if c < best { best = c }
        c = tsp_route_cost(d01,d02,d03,d04,d12,d13,d14,d23,d24,d34, 0,3,1,4,2)
        if c < best { best = c }
        c = tsp_route_cost(d01,d02,d03,d04,d12,d13,d14,d23,d24,d34, 0,3,2,1,4)
        if c < best { best = c }
        c = tsp_route_cost(d01,d02,d03,d04,d12,d13,d14,d23,d24,d34, 0,3,2,4,1)
        if c < best { best = c }
        c = tsp_route_cost(d01,d02,d03,d04,d12,d13,d14,d23,d24,d34, 0,3,4,1,2)
        if c < best { best = c }
        c = tsp_route_cost(d01,d02,d03,d04,d12,d13,d14,d23,d24,d34, 0,3,4,2,1)
        if c < best { best = c }
        c = tsp_route_cost(d01,d02,d03,d04,d12,d13,d14,d23,d24,d34, 0,4,1,2,3)
        if c < best { best = c }
        c = tsp_route_cost(d01,d02,d03,d04,d12,d13,d14,d23,d24,d34, 0,4,1,3,2)
        if c < best { best = c }
        c = tsp_route_cost(d01,d02,d03,d04,d12,d13,d14,d23,d24,d34, 0,4,2,1,3)
        if c < best { best = c }
        c = tsp_route_cost(d01,d02,d03,d04,d12,d13,d14,d23,d24,d34, 0,4,2,3,1)
        if c < best { best = c }
        c = tsp_route_cost(d01,d02,d03,d04,d12,d13,d14,d23,d24,d34, 0,4,3,1,2)
        if c < best { best = c }
        c = tsp_route_cost(d01,d02,d03,d04,d12,d13,d14,d23,d24,d34, 0,4,3,2,1)
        if c < best { best = c }

        total_min = total_min + best
        if total_min > 1000000000 {
            total_min = mod_int(total_min, 1000000)
        }
        iter = iter + 1
    }

    printf("  total minimum TSP cost = ")
    printf(total_min)
    printf("\n\n")
}

// ============================================================
// TEST 7: Union-Find simulation (procedural, 100K)
// ============================================================

void bench_union_find() {
    printf("--- Test 7: Union-Find 6 Nodes (100K operations) ---\n")

    int total_components = 0
    int iter = 0
    int seed = 53

    while iter < 100000 {
        // 6 nodes, each is its own parent initially
        int p0 = 0  int p1 = 1  int p2 = 2  int p3 = 3  int p4 = 4  int p5 = 5

        // Generate 3 random union operations
        seed = seed * 1103515245 + 12345  int s = seed  if s < 0 { s = 0 - s }
        int u1a = mod_int(s, 6)
        seed = seed * 1103515245 + 12345  s = seed  if s < 0 { s = 0 - s }
        int u1b = mod_int(s, 6)
        seed = seed * 1103515245 + 12345  s = seed  if s < 0 { s = 0 - s }
        int u2a = mod_int(s, 6)
        seed = seed * 1103515245 + 12345  s = seed  if s < 0 { s = 0 - s }
        int u2b = mod_int(s, 6)
        seed = seed * 1103515245 + 12345  s = seed  if s < 0 { s = 0 - s }
        int u3a = mod_int(s, 6)
        seed = seed * 1103515245 + 12345  s = seed  if s < 0 { s = 0 - s }
        int u3b = mod_int(s, 6)

        // Find root of a node (inline, max 2 hops)
        // Union: set parent of root(a) to root(b)
        // Simplified: just set parent directly
        // Union 1
        int ra = u1a  int rb = u1b
        // Find root of ra
        if ra == 0 { ra = p0 }  if ra == 1 { ra = p1 }  if ra == 2 { ra = p2 }
        if ra == 3 { ra = p3 }  if ra == 4 { ra = p4 }  if ra == 5 { ra = p5 }
        if rb == 0 { rb = p0 }  if rb == 1 { rb = p1 }  if rb == 2 { rb = p2 }
        if rb == 3 { rb = p3 }  if rb == 4 { rb = p4 }  if rb == 5 { rb = p5 }
        if ra != rb {
            if ra == 0 { p0 = rb } if ra == 1 { p1 = rb } if ra == 2 { p2 = rb }
            if ra == 3 { p3 = rb } if ra == 4 { p4 = rb } if ra == 5 { p5 = rb }
        }

        // Union 2
        ra = u2a  rb = u2b
        if ra == 0 { ra = p0 }  if ra == 1 { ra = p1 }  if ra == 2 { ra = p2 }
        if ra == 3 { ra = p3 }  if ra == 4 { ra = p4 }  if ra == 5 { ra = p5 }
        if rb == 0 { rb = p0 }  if rb == 1 { rb = p1 }  if rb == 2 { rb = p2 }
        if rb == 3 { rb = p3 }  if rb == 4 { rb = p4 }  if rb == 5 { rb = p5 }
        if ra != rb {
            if ra == 0 { p0 = rb } if ra == 1 { p1 = rb } if ra == 2 { p2 = rb }
            if ra == 3 { p3 = rb } if ra == 4 { p4 = rb } if ra == 5 { p5 = rb }
        }

        // Union 3
        ra = u3a  rb = u3b
        if ra == 0 { ra = p0 }  if ra == 1 { ra = p1 }  if ra == 2 { ra = p2 }
        if ra == 3 { ra = p3 }  if ra == 4 { ra = p4 }  if ra == 5 { ra = p5 }
        if rb == 0 { rb = p0 }  if rb == 1 { rb = p1 }  if rb == 2 { rb = p2 }
        if rb == 3 { rb = p3 }  if rb == 4 { rb = p4 }  if rb == 5 { rb = p5 }
        if ra != rb {
            if ra == 0 { p0 = rb } if ra == 1 { p1 = rb } if ra == 2 { p2 = rb }
            if ra == 3 { p3 = rb } if ra == 4 { p4 = rb } if ra == 5 { p5 = rb }
        }

        // Count distinct components (count nodes that are their own root)
        int comp = 0
        if p0 == 0 { comp = comp + 1 }
        if p1 == 1 { comp = comp + 1 }
        if p2 == 2 { comp = comp + 1 }
        if p3 == 3 { comp = comp + 1 }
        if p4 == 4 { comp = comp + 1 }
        if p5 == 5 { comp = comp + 1 }

        total_components = total_components + comp
        iter = iter + 1
    }

    printf("  total components counted = ")
    printf(total_components)
    printf("\n\n")
}

// ============================================================
// TEST 8: Chebyshev distance (procedural, 10M)
// ============================================================

void bench_chebyshev() {
    printf("--- Test 8: Chebyshev Distance (10M pairs) ---\n")

    int total = 0
    int seed = 11
    int i = 0

    while i < 10000000 {
        seed = seed * 1103515245 + 12345  int s = seed  if s < 0 { s = 0 - s }
        int x1 = mod_int(s, 1000)
        seed = seed * 1103515245 + 12345  s = seed  if s < 0 { s = 0 - s }
        int y1 = mod_int(s, 1000)
        seed = seed * 1103515245 + 12345  s = seed  if s < 0 { s = 0 - s }
        int x2 = mod_int(s, 1000)
        seed = seed * 1103515245 + 12345  s = seed  if s < 0 { s = 0 - s }
        int y2 = mod_int(s, 1000)

        int dx = abs_val(x2 - x1)
        int dy = abs_val(y2 - y1)
        int dist = dx
        if dy > dist { dist = dy }

        total = total + dist
        if total > 1000000000 {
            total = mod_int(total, 1000000)
        }
        i = i + 1
    }

    printf("  total chebyshev distance = ")
    printf(total)
    printf("\n\n")
}

// ============================================================
// Main — run all pathfinding benchmarks
// ============================================================
int main() {
    printf("============================================================\n")
    printf("ADead-BIB Benchmark Suite -- 13: Pathfinding & Graphs\n")
    printf("============================================================\n\n")

    bench_manhattan()
    bench_flood_fill()
    bench_dijkstra()
    bench_astar_heuristic()
    bench_bfs()
    bench_tsp()
    bench_union_find()
    bench_chebyshev()

    printf("============================================================\n")
    printf("All pathfinding benchmarks complete.\n")
    printf("============================================================\n")
    return 0
}
