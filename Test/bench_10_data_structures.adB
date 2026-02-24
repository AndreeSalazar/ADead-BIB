// ============================================================
// ADead-BIB Benchmark Suite — 10: Data Structures (Simulated)
// ============================================================
// Simulates data structures using scalar variables and
// recursion. Combines procedural C-style and OOP-light.
//
// Compilar: adeadc bench_10_data_structures.adB -o bench10.exe
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

// ============================================================
// TEST 1: Stack simulation (OOP-light)
// 16-element stack with push/pop via variables
// ============================================================

// Stack "object": s0-s15 = storage, sp = stack pointer
// Returns new sp after push (stores value at position sp)
int stack_push_val(int sp, int val, int pos) {
    // We return the value that should go into position pos
    if sp == pos {
        return val
    }
    return 0 - 1  // sentinel: don't change
}

void bench_stack() {
    printf("--- Test 1: Stack Simulation (OOP-light, 100K ops) ---\n")

    int s0 = 0  int s1 = 0  int s2 = 0  int s3 = 0
    int s4 = 0  int s5 = 0  int s6 = 0  int s7 = 0
    int s8 = 0  int s9 = 0  int s10 = 0 int s11 = 0
    int s12 = 0 int s13 = 0 int s14 = 0 int s15 = 0
    int sp = 0
    int checksum = 0
    int i = 0

    while i < 100000 {
        int val = mod_int(i * 7919 + 13, 10000)

        // Push phase: push if sp < 16
        if sp < 16 {
            if sp == 0 { s0 = val }
            if sp == 1 { s1 = val }
            if sp == 2 { s2 = val }
            if sp == 3 { s3 = val }
            if sp == 4 { s4 = val }
            if sp == 5 { s5 = val }
            if sp == 6 { s6 = val }
            if sp == 7 { s7 = val }
            if sp == 8 { s8 = val }
            if sp == 9 { s9 = val }
            if sp == 10 { s10 = val }
            if sp == 11 { s11 = val }
            if sp == 12 { s12 = val }
            if sp == 13 { s13 = val }
            if sp == 14 { s14 = val }
            if sp == 15 { s15 = val }
            sp = sp + 1
        }

        // Pop phase: pop if sp > 8 (keep stack from overflowing)
        if sp > 8 {
            sp = sp - 1
            int popped = 0
            if sp == 0 { popped = s0 }
            if sp == 1 { popped = s1 }
            if sp == 2 { popped = s2 }
            if sp == 3 { popped = s3 }
            if sp == 4 { popped = s4 }
            if sp == 5 { popped = s5 }
            if sp == 6 { popped = s6 }
            if sp == 7 { popped = s7 }
            if sp == 8 { popped = s8 }
            if sp == 9 { popped = s9 }
            if sp == 10 { popped = s10 }
            if sp == 11 { popped = s11 }
            if sp == 12 { popped = s12 }
            if sp == 13 { popped = s13 }
            if sp == 14 { popped = s14 }
            if sp == 15 { popped = s15 }
            checksum = checksum + popped
            if checksum > 1000000000 {
                checksum = mod_int(checksum, 1000000)
            }
        }

        i = i + 1
    }

    printf("  checksum = ")
    printf(checksum)
    printf(", final sp = ")
    printf(sp)
    printf("\n\n")
}

// ============================================================
// TEST 2: Queue simulation (OOP-light)
// Circular buffer of 8 elements
// ============================================================

void bench_queue() {
    printf("--- Test 2: Queue / Circular Buffer (OOP-light, 100K) ---\n")

    int q0 = 0  int q1 = 0  int q2 = 0  int q3 = 0
    int q4 = 0  int q5 = 0  int q6 = 0  int q7 = 0
    int head = 0
    int tail = 0
    int count = 0
    int checksum = 0
    int i = 0

    while i < 100000 {
        int val = mod_int(i * 2971 + 37, 10000)

        // Enqueue if not full
        if count < 8 {
            int pos = mod_int(tail, 8)
            if pos == 0 { q0 = val }
            if pos == 1 { q1 = val }
            if pos == 2 { q2 = val }
            if pos == 3 { q3 = val }
            if pos == 4 { q4 = val }
            if pos == 5 { q5 = val }
            if pos == 6 { q6 = val }
            if pos == 7 { q7 = val }
            tail = tail + 1
            count = count + 1
        }

        // Dequeue if count > 4
        if count > 4 {
            int pos = mod_int(head, 8)
            int dequeued = 0
            if pos == 0 { dequeued = q0 }
            if pos == 1 { dequeued = q1 }
            if pos == 2 { dequeued = q2 }
            if pos == 3 { dequeued = q3 }
            if pos == 4 { dequeued = q4 }
            if pos == 5 { dequeued = q5 }
            if pos == 6 { dequeued = q6 }
            if pos == 7 { dequeued = q7 }
            head = head + 1
            count = count - 1
            checksum = checksum + dequeued
            if checksum > 1000000000 {
                checksum = mod_int(checksum, 1000000)
            }
        }

        i = i + 1
    }

    printf("  checksum = ")
    printf(checksum)
    printf(", count = ")
    printf(count)
    printf("\n\n")
}

// ============================================================
// TEST 3: Hash table simulation (procedural)
// 16 "buckets" with linear probing
// ============================================================

int hash_key(int key) {
    int h = key * 2654435761
    if h < 0 {
        h = 0 - h
    }
    return mod_int(h, 16)
}

void bench_hash_table() {
    printf("--- Test 3: Hash Table (procedural, 100K inserts) ---\n")

    int b0 = 0  int b1 = 0  int b2 = 0  int b3 = 0
    int b4 = 0  int b5 = 0  int b6 = 0  int b7 = 0
    int b8 = 0  int b9 = 0  int b10 = 0 int b11 = 0
    int b12 = 0 int b13 = 0 int b14 = 0 int b15 = 0
    int collisions = 0
    int i = 0

    while i < 100000 {
        int key = mod_int(i * 7919 + 1, 1000000)
        int bucket = hash_key(key)
        int val = mod_int(key, 10000) + 1

        // Store in bucket (overwrite)
        if bucket == 0 { b0 = val }
        if bucket == 1 { b1 = val }
        if bucket == 2 { b2 = val }
        if bucket == 3 { b3 = val }
        if bucket == 4 { b4 = val }
        if bucket == 5 { b5 = val }
        if bucket == 6 { b6 = val }
        if bucket == 7 { b7 = val }
        if bucket == 8 { b8 = val }
        if bucket == 9 { b9 = val }
        if bucket == 10 { b10 = val }
        if bucket == 11 { b11 = val }
        if bucket == 12 { b12 = val }
        if bucket == 13 { b13 = val }
        if bucket == 14 { b14 = val }
        if bucket == 15 { b15 = val }

        // Count "collisions" (same bucket as previous)
        int prev_bucket = hash_key(mod_int((i - 1) * 7919 + 1, 1000000))
        if bucket == prev_bucket {
            if i > 0 {
                collisions = collisions + 1
            }
        }

        i = i + 1
    }

    int total = b0 + b1 + b2 + b3 + b4 + b5 + b6 + b7 + b8 + b9 + b10 + b11 + b12 + b13 + b14 + b15
    printf("  bucket sum = ")
    printf(total)
    printf(", collisions = ")
    printf(collisions)
    printf("\n\n")
}

// ============================================================
// TEST 4: Priority queue / min-heap (OOP-light)
// 7-element heap using variables h0-h6
// ============================================================

void bench_priority_queue() {
    printf("--- Test 4: Priority Queue / Min-Heap (OOP-light, 10K) ---\n")

    int h0 = 999999  int h1 = 999999  int h2 = 999999
    int h3 = 999999  int h4 = 999999  int h5 = 999999  int h6 = 999999
    int heap_size = 0
    int checksum = 0
    int i = 0

    while i < 10000 {
        int val = mod_int(i * 6151 + 23, 10000)

        // Insert (push): add at end, bubble up
        if heap_size < 7 {
            // Place at heap_size position
            if heap_size == 0 { h0 = val }
            if heap_size == 1 { h1 = val }
            if heap_size == 2 { h2 = val }
            if heap_size == 3 { h3 = val }
            if heap_size == 4 { h4 = val }
            if heap_size == 5 { h5 = val }
            if heap_size == 6 { h6 = val }
            heap_size = heap_size + 1

            // Simple bubble-up: just fix top 3 levels
            // Swap h1/h0 if h1 < h0
            if h1 < h0 { int t = h0  h0 = h1  h1 = t }
            if h2 < h0 { int t = h0  h0 = h2  h2 = t }
            if h3 < h1 { int t = h1  h1 = h3  h3 = t }
            if h4 < h1 { int t = h1  h1 = h4  h4 = t }
            if h5 < h2 { int t = h2  h2 = h5  h5 = t }
            if h6 < h2 { int t = h2  h2 = h6  h6 = t }
            // Fix root again
            if h1 < h0 { int t = h0  h0 = h1  h1 = t }
            if h2 < h0 { int t = h0  h0 = h2  h2 = t }
        }

        // Extract min when full
        if heap_size >= 7 {
            checksum = checksum + h0
            if checksum > 1000000000 {
                checksum = mod_int(checksum, 1000000)
            }
            // Move last to root, shrink
            h0 = h6
            h6 = 999999
            heap_size = heap_size - 1
            // Re-heapify root
            if h1 < h2 {
                if h1 < h0 { int t = h0  h0 = h1  h1 = t }
            } else {
                if h2 < h0 { int t = h0  h0 = h2  h2 = t }
            }
        }

        i = i + 1
    }

    printf("  checksum = ")
    printf(checksum)
    printf(", heap_size = ")
    printf(heap_size)
    printf("\n\n")
}

// ============================================================
// TEST 5: Linked list via recursion (procedural)
// Build recursive "list", sum elements
// ============================================================

int list_sum(int depth, int seed) {
    if depth <= 0 {
        return 0
    }
    int val = mod_int(seed * 31 + 17, 10000)
    return val + list_sum(depth - 1, val)
}

void bench_linked_list() {
    printf("--- Test 5: Linked List via Recursion (procedural) ---\n")

    int total = 0
    int i = 0
    while i < 100 {
        int sum = list_sum(1000, i + 1)
        total = total + mod_int(sum, 100000)
        if total > 1000000000 {
            total = mod_int(total, 1000000)
        }
        i = i + 1
    }

    printf("  total = ")
    printf(total)
    printf("\n\n")
}

// ============================================================
// TEST 6: Binary tree traversal (procedural)
// Recursive complete binary tree, compute sum of all nodes
// ============================================================

int tree_sum(int node_val, int depth) {
    if depth <= 0 {
        return node_val
    }
    int left_val = node_val * 2 + 1
    int right_val = node_val * 2 + 2
    int left_sum = tree_sum(mod_int(left_val, 10000), depth - 1)
    int right_sum = tree_sum(mod_int(right_val, 10000), depth - 1)
    return mod_int(node_val + left_sum + right_sum, 1000000)
}

void bench_binary_tree() {
    printf("--- Test 6: Binary Tree Traversal (procedural, depth 15) ---\n")

    int result = tree_sum(1, 15)
    printf("  tree sum (mod 1M) = ")
    printf(result)
    printf("\n\n")
}

// ============================================================
// TEST 7: Ring buffer (OOP-light)
// 8-element circular write/read, 100K operations
// ============================================================

void bench_ring_buffer() {
    printf("--- Test 7: Ring Buffer (OOP-light, 100K ops) ---\n")

    int r0 = 0  int r1 = 0  int r2 = 0  int r3 = 0
    int r4 = 0  int r5 = 0  int r6 = 0  int r7 = 0
    int write_pos = 0
    int read_pos = 0
    int checksum = 0
    int i = 0

    while i < 100000 {
        int val = mod_int(i * 4217 + 7, 10000)
        int wp = mod_int(write_pos, 8)

        // Write
        if wp == 0 { r0 = val }
        if wp == 1 { r1 = val }
        if wp == 2 { r2 = val }
        if wp == 3 { r3 = val }
        if wp == 4 { r4 = val }
        if wp == 5 { r5 = val }
        if wp == 6 { r6 = val }
        if wp == 7 { r7 = val }
        write_pos = write_pos + 1

        // Read (2 positions behind)
        if i > 1 {
            int rp = mod_int(read_pos, 8)
            int read_val = 0
            if rp == 0 { read_val = r0 }
            if rp == 1 { read_val = r1 }
            if rp == 2 { read_val = r2 }
            if rp == 3 { read_val = r3 }
            if rp == 4 { read_val = r4 }
            if rp == 5 { read_val = r5 }
            if rp == 6 { read_val = r6 }
            if rp == 7 { read_val = r7 }
            read_pos = read_pos + 1
            checksum = checksum + read_val
            if checksum > 1000000000 {
                checksum = mod_int(checksum, 1000000)
            }
        }

        i = i + 1
    }

    printf("  checksum = ")
    printf(checksum)
    printf("\n\n")
}

// ============================================================
// TEST 8: LRU cache simulation (procedural)
// 4-element cache, 100K lookups
// ============================================================

void bench_lru_cache() {
    printf("--- Test 8: LRU Cache (procedural, 4 slots, 100K lookups) ---\n")

    // Cache: key-value pairs (k0,v0)..(k3,v3) + age
    int k0 = -1  int v0 = 0  int age0 = 0
    int k1 = -1  int v1 = 0  int age1 = 0
    int k2 = -1  int v2 = 0  int age2 = 0
    int k3 = -1  int v3 = 0  int age3 = 0
    int hits = 0
    int misses = 0
    int i = 0

    while i < 100000 {
        // Generate key with some locality
        int key = mod_int(i * 2971 + 13, 20)
        int found = 0

        // Search cache
        if k0 == key { found = 1  age0 = i  hits = hits + 1 }
        if found == 0 {
            if k1 == key { found = 1  age1 = i  hits = hits + 1 }
        }
        if found == 0 {
            if k2 == key { found = 1  age2 = i  hits = hits + 1 }
        }
        if found == 0 {
            if k3 == key { found = 1  age3 = i  hits = hits + 1 }
        }

        // Cache miss: evict LRU
        if found == 0 {
            misses = misses + 1
            // Find oldest (min age)
            int min_age = age0
            int evict = 0
            if age1 < min_age { min_age = age1  evict = 1 }
            if age2 < min_age { min_age = age2  evict = 2 }
            if age3 < min_age { min_age = age3  evict = 3 }

            int val = key * 100 + 1
            if evict == 0 { k0 = key  v0 = val  age0 = i }
            if evict == 1 { k1 = key  v1 = val  age1 = i }
            if evict == 2 { k2 = key  v2 = val  age2 = i }
            if evict == 3 { k3 = key  v3 = val  age3 = i }
        }

        i = i + 1
    }

    printf("  hits = ")
    printf(hits)
    printf(", misses = ")
    printf(misses)
    printf("\n\n")
}

// ============================================================
// TEST 9: Accumulator pattern (OOP-light)
// Track count, sum, min, max for multiple "objects"
// ============================================================

void bench_accumulator() {
    printf("--- Test 9: Accumulator Pattern (OOP-light, 100K values) ---\n")

    // Accumulator A
    int a_count = 0  int a_sum = 0  int a_min = 2147483647  int a_max = 0
    // Accumulator B
    int b_count = 0  int b_sum = 0  int b_min = 2147483647  int b_max = 0

    int i = 0
    while i < 100000 {
        int val = mod_int(i * 6271 + 31, 10000)

        // Dispatch to accumulator A (even) or B (odd)
        int parity = mod_int(i, 2)

        if parity == 0 {
            a_count = a_count + 1
            a_sum = a_sum + val
            if a_sum > 1000000000 {
                a_sum = mod_int(a_sum, 1000000)
            }
            if val < a_min { a_min = val }
            if val > a_max { a_max = val }
        } else {
            b_count = b_count + 1
            b_sum = b_sum + val
            if b_sum > 1000000000 {
                b_sum = mod_int(b_sum, 1000000)
            }
            if val < b_min { b_min = val }
            if val > b_max { b_max = val }
        }

        i = i + 1
    }

    printf("  A: count=")
    printf(a_count)
    printf(" sum=")
    printf(a_sum)
    printf(" min=")
    printf(a_min)
    printf(" max=")
    printf(a_max)
    printf("\n")
    printf("  B: count=")
    printf(b_count)
    printf(" sum=")
    printf(b_sum)
    printf(" min=")
    printf(b_min)
    printf(" max=")
    printf(b_max)
    printf("\n\n")
}

// ============================================================
// TEST 10: State machine with history (OOP-light)
// 6 states, 1M transitions, track history
// ============================================================

void bench_state_machine() {
    printf("--- Test 10: State Machine + History (OOP-light, 1M) ---\n")

    int state = 0
    int s0_count = 0  int s1_count = 0  int s2_count = 0
    int s3_count = 0  int s4_count = 0  int s5_count = 0
    int transitions = 0

    // History buffer (last 4 states)
    int hist0 = 0  int hist1 = 0  int hist2 = 0  int hist3 = 0
    int hist_pos = 0
    int history_sum = 0

    int i = 0
    while i < 1000000 {
        int input = mod_int(i * 2971 + 7, 10)

        // Record history
        int hp = mod_int(hist_pos, 4)
        if hp == 0 { hist0 = state }
        if hp == 1 { hist1 = state }
        if hp == 2 { hist2 = state }
        if hp == 3 { hist3 = state }
        hist_pos = hist_pos + 1
        history_sum = history_sum + state

        if history_sum > 1000000000 {
            history_sum = mod_int(history_sum, 1000000)
        }

        // State transitions
        if state == 0 {
            s0_count = s0_count + 1
            if input < 3 { state = 1 }
            else { if input < 6 { state = 2 } else { state = 3 } }
        } else {
            if state == 1 {
                s1_count = s1_count + 1
                if input < 4 { state = 0 }
                else { if input < 7 { state = 4 } else { state = 2 } }
            } else {
                if state == 2 {
                    s2_count = s2_count + 1
                    if input < 5 { state = 5 }
                    else { state = 0 }
                } else {
                    if state == 3 {
                        s3_count = s3_count + 1
                        if input < 3 { state = 1 }
                        else { if input < 8 { state = 4 } else { state = 5 } }
                    } else {
                        if state == 4 {
                            s4_count = s4_count + 1
                            if input < 5 { state = 0 }
                            else { state = 3 }
                        } else {
                            s5_count = s5_count + 1
                            if input < 4 { state = 2 }
                            else { if input < 7 { state = 1 } else { state = 0 } }
                        }
                    }
                }
            }
        }

        transitions = transitions + 1
        i = i + 1
    }

    printf("  States: s0=")
    printf(s0_count)
    printf(" s1=")
    printf(s1_count)
    printf(" s2=")
    printf(s2_count)
    printf(" s3=")
    printf(s3_count)
    printf(" s4=")
    printf(s4_count)
    printf(" s5=")
    printf(s5_count)
    printf("\n  transitions=")
    printf(transitions)
    printf(" history_sum=")
    printf(history_sum)
    printf("\n\n")
}

// ============================================================
// Main — run all data structure benchmarks
// ============================================================
int main() {
    printf("============================================================\n")
    printf("ADead-BIB Benchmark Suite -- 10: Data Structures\n")
    printf("============================================================\n\n")

    bench_stack()
    bench_queue()
    bench_hash_table()
    bench_priority_queue()
    bench_linked_list()
    bench_binary_tree()
    bench_ring_buffer()
    bench_lru_cache()
    bench_accumulator()
    bench_state_machine()

    printf("============================================================\n")
    printf("All data structure benchmarks complete.\n")
    printf("============================================================\n")
    return 0
}
