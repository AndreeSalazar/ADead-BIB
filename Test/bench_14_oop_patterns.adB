// ============================================================
// ADead-BIB Benchmark Suite — 14: OOP-Light Patterns
// ============================================================
// Demonstrates OOP design patterns in a procedural language.
// Shows C-style "struct + methods" and Rust-style "impl" patterns
// using function groups and parameter passing.
//
// This tests the compiler's ability to optimize function-heavy
// code patterns common in real-world applications.
// ============================================================

// ============================================================
// Helper: modulo (no % operator)
// ============================================================

int mod_int(int a, int b) {
    return a - (a / b) * b
}

// ============================================================
// Helper: integer square root (Newton's method)
// ============================================================

int isqrt(int n) {
    if n < 2 {
        return n
    }
    int x = n
    int y = (x + 1) / 2
    while y < x {
        x = y
        y = (x + n / x) / 2
    }
    return x
}

// ============================================================
// Helper: absolute value
// ============================================================

int abs_int(int x) {
    if x < 0 {
        return 0 - x
    }
    return x
}

// ============================================================
// TEST 1: Point2D "class" (OOP-light)
// ============================================================
// Functions: point_create, point_distance_sq, point_translate,
// point_scale. Compute distances between 1M point pairs.
// A Point2D is represented as (x, y) passed as parameters.
// ============================================================

int point_create_x(int id) {
    return mod_int(id * 7 + 13, 1000)
}

int point_create_y(int id) {
    return mod_int(id * 11 + 29, 1000)
}

int point_translate_x(int px, int dx) {
    return px + dx
}

int point_translate_y(int py, int dy) {
    return py + dy
}

int point_scale_x(int px, int factor) {
    return px * factor
}

int point_scale_y(int py, int factor) {
    return py * factor
}

int point_distance_sq(int x1, int y1, int x2, int y2) {
    int dx = x1 - x2
    int dy = y1 - y2
    return dx * dx + dy * dy
}

void test_point2d() {
    printf("--- Test 1: Point2D class (1M point pairs) ---\n")
    int i = 0
    int total_dist = 0
    while i < 1000000 {
        int x1 = point_create_x(i)
        int y1 = point_create_y(i)
        int x2 = point_create_x(i + 1)
        int y2 = point_create_y(i + 1)

        x1 = point_translate_x(x1, 10)
        y1 = point_translate_y(y1, 20)

        x2 = point_scale_x(x2, 2)
        y2 = point_scale_y(y2, 2)

        int dsq = point_distance_sq(x1, y1, x2, y2)
        total_dist = total_dist + isqrt(dsq)
        if total_dist > 1000000000 {
            total_dist = mod_int(total_dist, 1000000)
        }
        i = i + 1
    }
    printf("  total distance sum = ")
    printf(total_dist)
    printf("\n\n")
}

// ============================================================
// TEST 2: Vector3D "class" (OOP-light)
// ============================================================
// Functions: vec3_create, vec3_add, vec3_dot, vec3_cross,
// vec3_length_sq, vec3_normalize_fixed. Process 1M vectors.
// A Vec3 is represented as (x, y, z).
// ============================================================

int vec3_create_x(int id) {
    return mod_int(id * 3 + 7, 100)
}

int vec3_create_y(int id) {
    return mod_int(id * 5 + 11, 100)
}

int vec3_create_z(int id) {
    return mod_int(id * 7 + 13, 100)
}

int vec3_add_x(int ax, int bx) {
    return ax + bx
}

int vec3_add_y(int ay, int by) {
    return ay + by
}

int vec3_add_z(int az, int bz) {
    return az + bz
}

int vec3_dot(int ax, int ay, int az, int bx, int by, int bz) {
    return ax * bx + ay * by + az * bz
}

int vec3_cross_x(int ay, int az, int by, int bz) {
    return ay * bz - az * by
}

int vec3_cross_y(int ax, int az, int bx, int bz) {
    return az * bx - ax * bz
}

int vec3_cross_z(int ax, int ay, int bx, int by) {
    return ax * by - ay * bx
}

int vec3_length_sq(int x, int y, int z) {
    return x * x + y * y + z * z
}

int vec3_normalize_fixed_x(int x, int y, int z) {
    int len = isqrt(vec3_length_sq(x, y, z))
    if len == 0 {
        return 0
    }
    return (x * 1000) / len
}

int vec3_normalize_fixed_y(int x, int y, int z) {
    int len = isqrt(vec3_length_sq(x, y, z))
    if len == 0 {
        return 0
    }
    return (y * 1000) / len
}

int vec3_normalize_fixed_z(int x, int y, int z) {
    int len = isqrt(vec3_length_sq(x, y, z))
    if len == 0 {
        return 0
    }
    return (z * 1000) / len
}

void test_vector3d() {
    printf("--- Test 2: Vector3D class (1M vectors) ---\n")
    int i = 0
    int checksum = 0
    while i < 1000000 {
        int ax = vec3_create_x(i)
        int ay = vec3_create_y(i)
        int az = vec3_create_z(i)
        int bx = vec3_create_x(i + 1)
        int by = vec3_create_y(i + 1)
        int bz = vec3_create_z(i + 1)

        int sx = vec3_add_x(ax, bx)
        int sy = vec3_add_y(ay, by)
        int sz = vec3_add_z(az, bz)

        int dot = vec3_dot(ax, ay, az, bx, by, bz)

        int cx = vec3_cross_x(ay, az, by, bz)
        int cy = vec3_cross_y(ax, az, bx, bz)
        int cz = vec3_cross_z(ax, ay, bx, by)

        int nx = vec3_normalize_fixed_x(sx, sy, sz)

        checksum = checksum + dot + cx + cy + cz + nx
        if checksum > 1000000000 {
            checksum = mod_int(checksum, 1000000)
        }
        if checksum < -1000000000 {
            checksum = 0 - mod_int(0 - checksum, 1000000)
        }
        i = i + 1
    }
    printf("  vector checksum = ")
    printf(checksum)
    printf("\n\n")
}

// ============================================================
// TEST 3: Counter "object" (OOP-light)
// ============================================================
// Functions: counter_new, counter_increment, counter_decrement,
// counter_reset, counter_get. 10M operations on multiple counters.
// A Counter is represented by its current value.
// ============================================================

int counter_new() {
    return 0
}

int counter_increment(int val) {
    return val + 1
}

int counter_decrement(int val) {
    return val - 1
}

int counter_reset() {
    return 0
}

int counter_get(int val) {
    return val
}

void test_counter() {
    printf("--- Test 3: Counter object (10M operations) ---\n")
    int c1 = counter_new()
    int c2 = counter_new()
    int c3 = counter_new()
    int i = 0
    while i < 10000000 {
        int op = mod_int(i, 5)
        if op == 0 {
            c1 = counter_increment(c1)
        }
        if op == 1 {
            c2 = counter_increment(c2)
        }
        if op == 2 {
            c3 = counter_decrement(c3)
        }
        if op == 3 {
            c1 = counter_increment(c1)
            c2 = counter_decrement(c2)
        }
        if op == 4 {
            c3 = counter_increment(c3)
        }
        i = i + 1
    }
    int total = counter_get(c1) + counter_get(c2) + counter_get(c3)
    printf("  c1 = ")
    printf(counter_get(c1))
    printf(", c2 = ")
    printf(counter_get(c2))
    printf(", c3 = ")
    printf(counter_get(c3))
    printf(", total = ")
    printf(total)
    printf("\n\n")
}

// ============================================================
// TEST 4: Builder pattern (OOP-light)
// ============================================================
// Simulate a configuration builder: builder_new, builder_set_width,
// builder_set_height, builder_set_depth, builder_build (returns
// computed volume). 1M builds.
// A Builder is represented as (width, height, depth).
// ============================================================

int builder_new_w() {
    return 1
}

int builder_new_h() {
    return 1
}

int builder_new_d() {
    return 1
}

int builder_set_width(int w) {
    return w
}

int builder_set_height(int h) {
    return h
}

int builder_set_depth(int d) {
    return d
}

int builder_build(int w, int h, int d) {
    return w * h * d
}

void test_builder() {
    printf("--- Test 4: Builder pattern (1M builds) ---\n")
    int i = 0
    int total_volume = 0
    while i < 1000000 {
        int w = builder_set_width(mod_int(i, 50) + 1)
        int h = builder_set_height(mod_int(i * 3, 40) + 1)
        int d = builder_set_depth(mod_int(i * 7, 30) + 1)
        int vol = builder_build(w, h, d)
        total_volume = total_volume + vol
        if total_volume > 1000000000 {
            total_volume = mod_int(total_volume, 1000000)
        }
        i = i + 1
    }
    printf("  total volume = ")
    printf(total_volume)
    printf("\n\n")
}

// ============================================================
// TEST 5: Strategy pattern (OOP-light)
// ============================================================
// Three "strategies" for computation (add_strategy, mul_strategy,
// pow_strategy). Dispatch 1M operations based on strategy_id.
// ============================================================

int add_strategy(int a, int b) {
    return a + b
}

int mul_strategy(int a, int b) {
    return a * b
}

int pow_strategy(int base, int exp) {
    int result = 1
    int e = exp
    while e > 0 {
        result = result * base
        if result > 1000000000 {
            result = mod_int(result, 1000000)
        }
        e = e - 1
    }
    return result
}

int strategy_dispatch(int strategy_id, int a, int b) {
    if strategy_id == 0 {
        return add_strategy(a, b)
    }
    if strategy_id == 1 {
        return mul_strategy(a, b)
    }
    if strategy_id == 2 {
        return pow_strategy(a, b)
    }
    return 0
}

void test_strategy() {
    printf("--- Test 5: Strategy pattern (1M dispatches) ---\n")
    int i = 0
    int acc = 0
    while i < 1000000 {
        int sid = mod_int(i, 3)
        int a = mod_int(i, 10) + 1
        int b = mod_int(i, 5) + 1
        int result = strategy_dispatch(sid, a, b)
        acc = acc + result
        if acc > 1000000000 {
            acc = mod_int(acc, 1000000)
        }
        i = i + 1
    }
    printf("  accumulated = ")
    printf(acc)
    printf("\n\n")
}

// ============================================================
// TEST 6: Observer pattern (OOP-light)
// ============================================================
// Simulate event dispatch: event_fire triggers N "observers"
// (functions). Process 1M events with 4 observers each.
// ============================================================

int observer_log(int event_id, int data) {
    return data + 1
}

int observer_validate(int event_id, int data) {
    if data > 500 {
        return data - 100
    }
    return data + 50
}

int observer_transform(int event_id, int data) {
    return data * 2 + mod_int(event_id, 7)
}

int observer_aggregate(int event_id, int data, int running_total) {
    return running_total + data
}

int event_fire(int event_id, int data, int running_total) {
    int d = observer_log(event_id, data)
    d = observer_validate(event_id, d)
    d = observer_transform(event_id, d)
    int result = observer_aggregate(event_id, d, running_total)
    return result
}

void test_observer() {
    printf("--- Test 6: Observer pattern (1M events, 4 observers) ---\n")
    int i = 0
    int running = 0
    while i < 1000000 {
        int data = mod_int(i * 13, 1000)
        running = event_fire(i, data, running)
        if running > 1000000000 {
            running = mod_int(running, 1000000)
        }
        i = i + 1
    }
    printf("  running total = ")
    printf(running)
    printf("\n\n")
}

// ============================================================
// TEST 7: State pattern (OOP-light)
// ============================================================
// Traffic light simulation: 3 states (red=0, yellow=1, green=2),
// each with different duration and transition rules.
// 1M transitions.
// ============================================================

int state_duration(int state) {
    if state == 0 {
        return 30
    }
    if state == 1 {
        return 5
    }
    if state == 2 {
        return 25
    }
    return 0
}

int state_next(int state) {
    if state == 0 {
        return 2
    }
    if state == 1 {
        return 0
    }
    if state == 2 {
        return 1
    }
    return 0
}

int state_action(int state, int tick) {
    if state == 0 {
        return tick + 10
    }
    if state == 1 {
        return tick + 3
    }
    if state == 2 {
        return tick + 7
    }
    return tick
}

void test_state_machine() {
    printf("--- Test 7: State pattern / traffic light (1M transitions) ---\n")
    int state = 0
    int timer = 0
    int tick = 0
    int transitions = 0
    int action_sum = 0
    while transitions < 1000000 {
        int dur = state_duration(state)
        timer = timer + 1
        action_sum = action_sum + state_action(state, tick)
        if action_sum > 1000000000 {
            action_sum = mod_int(action_sum, 1000000)
        }
        if timer >= dur {
            state = state_next(state)
            timer = 0
            transitions = transitions + 1
        }
        tick = tick + 1
    }
    printf("  final state = ")
    printf(state)
    printf(", action_sum = ")
    printf(action_sum)
    printf(", ticks = ")
    printf(tick)
    printf("\n\n")
}

// ============================================================
// TEST 8: Iterator pattern (OOP-light)
// ============================================================
// Functions: iter_new, iter_next, iter_has_next, iter_current.
// Iterate over generated sequences (triangular numbers). 1M items.
// An Iterator is (current_index, current_value, max).
// ============================================================

int iter_new_index() {
    return 0
}

int iter_new_value() {
    return 0
}

int iter_has_next(int index, int max) {
    if index < max {
        return 1
    }
    return 0
}

int iter_next_index(int index) {
    return index + 1
}

int iter_next_value(int index, int value) {
    return value + index + 1
}

int iter_current(int value) {
    return value
}

void test_iterator() {
    printf("--- Test 8: Iterator pattern (1M items) ---\n")
    int idx = iter_new_index()
    int val = iter_new_value()
    int max = 1000000
    int checksum = 0
    while iter_has_next(idx, max) == 1 {
        val = iter_next_value(idx, val)
        idx = iter_next_index(idx)
        int cur = iter_current(val)
        checksum = checksum + mod_int(cur, 1000)
        if checksum > 1000000000 {
            checksum = mod_int(checksum, 1000000)
        }
    }
    printf("  final value = ")
    printf(val)
    printf(", checksum = ")
    printf(checksum)
    printf("\n\n")
}

// ============================================================
// TEST 9: Polymorphism via dispatch (OOP-light)
// ============================================================
// Simulate virtual method dispatch: shape_area(type, w, h)
// dispatches to circle_area, rect_area, triangle_area based
// on type. 1M calls.
// type: 0 = circle, 1 = rectangle, 2 = triangle
// ============================================================

int circle_area(int radius) {
    return 314 * radius * radius / 100
}

int rect_area(int w, int h) {
    return w * h
}

int triangle_area(int base, int height) {
    return base * height / 2
}

int shape_area(int shape_type, int w, int h) {
    if shape_type == 0 {
        return circle_area(w)
    }
    if shape_type == 1 {
        return rect_area(w, h)
    }
    if shape_type == 2 {
        return triangle_area(w, h)
    }
    return 0
}

void test_polymorphism() {
    printf("--- Test 9: Polymorphism via dispatch (1M calls) ---\n")
    int i = 0
    int total_area = 0
    while i < 1000000 {
        int stype = mod_int(i, 3)
        int w = mod_int(i, 100) + 1
        int h = mod_int(i * 7, 80) + 1
        int a = shape_area(stype, w, h)
        total_area = total_area + a
        if total_area > 1000000000 {
            total_area = mod_int(total_area, 1000000)
        }
        i = i + 1
    }
    printf("  total area = ")
    printf(total_area)
    printf("\n\n")
}

// ============================================================
// TEST 10: RAII resource pattern (OOP-light)
// ============================================================
// Simulate resource acquire/use/release cycle: res_acquire,
// res_use, res_release. Track resource lifetime counts.
// 1M cycles.
// A Resource is (id, active_flag, use_count).
// ============================================================

int res_acquire_id(int request_id) {
    return request_id * 31 + 17
}

int res_acquire_active() {
    return 1
}

int res_acquire_uses() {
    return 0
}

int res_use(int res_id, int use_count, int data) {
    return use_count + 1
}

int res_compute(int res_id, int data) {
    return res_id + data * 3
}

int res_release_active() {
    return 0
}

int res_is_active(int active) {
    return active
}

void test_raii() {
    printf("--- Test 10: RAII resource pattern (1M cycles) ---\n")
    int i = 0
    int total_uses = 0
    int total_compute = 0
    int active_count = 0
    int release_count = 0
    while i < 1000000 {
        int rid = res_acquire_id(i)
        int active = res_acquire_active()
        int uses = res_acquire_uses()
        active_count = active_count + 1

        int data = mod_int(i, 100) + 1
        uses = res_use(rid, uses, data)
        int comp = res_compute(rid, data)
        uses = res_use(rid, uses, data)
        comp = comp + res_compute(rid, data + 1)
        uses = res_use(rid, uses, data)

        total_uses = total_uses + uses
        total_compute = total_compute + comp
        if total_compute > 1000000000 {
            total_compute = mod_int(total_compute, 1000000)
        }

        active = res_release_active()
        release_count = release_count + 1
        i = i + 1
    }
    printf("  total uses = ")
    printf(total_uses)
    printf(", total compute = ")
    printf(total_compute)
    printf(", acquired = ")
    printf(active_count)
    printf(", released = ")
    printf(release_count)
    printf("\n\n")
}

// ============================================================
// Main — run all OOP-light pattern benchmarks
// ============================================================

int main() {
    printf("============================================================\n")
    printf("ADead-BIB Benchmark Suite — 14: OOP-Light Patterns\n")
    printf("============================================================\n\n")

    test_point2d()
    test_vector3d()
    test_counter()
    test_builder()
    test_strategy()
    test_observer()
    test_state_machine()
    test_iterator()
    test_polymorphism()
    test_raii()

    printf("============================================================\n")
    printf("All OOP-light pattern benchmarks complete.\n")
    printf("============================================================\n")
    return 0
}
