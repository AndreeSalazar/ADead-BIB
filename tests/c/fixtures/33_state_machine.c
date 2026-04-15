// ============================================================
// Test 33: State Machine — FSM con function pointers y enums
// ============================================================
// ADead-BIB Test Canon — Pattern avanzado C
// Verifica: fn ptrs + enum + struct + switch combinados
// ============================================================

#include <stdio.h>

// --- States ---
enum State {
    STATE_IDLE,
    STATE_RUNNING,
    STATE_PAUSED,
    STATE_ERROR,
    STATE_DONE,
    STATE_COUNT
};

// --- Events ---
enum Event {
    EVENT_START,
    EVENT_PAUSE,
    EVENT_RESUME,
    EVENT_STOP,
    EVENT_ERROR,
    EVENT_RESET,
    EVENT_COUNT
};

const char *state_names[] = {"IDLE", "RUNNING", "PAUSED", "ERROR", "DONE"};
const char *event_names[] = {"START", "PAUSE", "RESUME", "STOP", "ERROR", "RESET"};

// --- Transition table ---
typedef enum State (*TransitionFn)(void *context);

struct Transition {
    enum State next_state;
    TransitionFn action;
};

// --- Context ---
struct MachineContext {
    int ticks;
    int error_count;
    char log[256];
    int log_pos;
};

void log_event(struct MachineContext *ctx, const char *msg) {
    while (*msg && ctx->log_pos < 250) {
        ctx->log[ctx->log_pos++] = *msg++;
    }
    ctx->log[ctx->log_pos++] = ';';
    ctx->log[ctx->log_pos] = '\0';
}

// --- Action functions ---
enum State action_start(void *ctx_ptr) {
    struct MachineContext *ctx = (struct MachineContext *)ctx_ptr;
    log_event(ctx, "started");
    ctx->ticks = 0;
    return STATE_RUNNING;
}

enum State action_pause(void *ctx_ptr) {
    struct MachineContext *ctx = (struct MachineContext *)ctx_ptr;
    log_event(ctx, "paused");
    return STATE_PAUSED;
}

enum State action_resume(void *ctx_ptr) {
    struct MachineContext *ctx = (struct MachineContext *)ctx_ptr;
    log_event(ctx, "resumed");
    return STATE_RUNNING;
}

enum State action_stop(void *ctx_ptr) {
    struct MachineContext *ctx = (struct MachineContext *)ctx_ptr;
    log_event(ctx, "stopped");
    return STATE_DONE;
}

enum State action_error(void *ctx_ptr) {
    struct MachineContext *ctx = (struct MachineContext *)ctx_ptr;
    ctx->error_count++;
    log_event(ctx, "error");
    return STATE_ERROR;
}

enum State action_reset(void *ctx_ptr) {
    struct MachineContext *ctx = (struct MachineContext *)ctx_ptr;
    log_event(ctx, "reset");
    ctx->ticks = 0;
    return STATE_IDLE;
}

// --- Process event via switch-based FSM ---
enum State fsm_process(enum State current, enum Event event,
                       struct MachineContext *ctx) {
    switch (current) {
        case STATE_IDLE:
            if (event == EVENT_START) return action_start(ctx);
            break;
        case STATE_RUNNING:
            if (event == EVENT_PAUSE) return action_pause(ctx);
            if (event == EVENT_STOP) return action_stop(ctx);
            if (event == EVENT_ERROR) return action_error(ctx);
            ctx->ticks++;
            break;
        case STATE_PAUSED:
            if (event == EVENT_RESUME) return action_resume(ctx);
            if (event == EVENT_STOP) return action_stop(ctx);
            break;
        case STATE_ERROR:
            if (event == EVENT_RESET) return action_reset(ctx);
            break;
        case STATE_DONE:
            if (event == EVENT_RESET) return action_reset(ctx);
            break;
        default:
            break;
    }
    return current;
}

int main() {
    struct MachineContext ctx;
    ctx.ticks = 0;
    ctx.error_count = 0;
    ctx.log_pos = 0;
    ctx.log[0] = '\0';

    enum State state = STATE_IDLE;
    printf("initial: %s\n", state_names[state]);

    // --- Sequence: start -> pause -> resume -> error -> reset -> start -> stop ---
    enum Event sequence[] = {
        EVENT_START, EVENT_PAUSE, EVENT_RESUME,
        EVENT_ERROR, EVENT_RESET, EVENT_START, EVENT_STOP
    };
    int n = 7;
    int i;
    for (i = 0; i < n; i++) {
        enum State prev = state;
        state = fsm_process(state, sequence[i], &ctx);
        printf("%s + %s -> %s\n",
               state_names[prev], event_names[sequence[i]], state_names[state]);
    }

    printf("\nfinal: %s\n", state_names[state]);
    printf("errors: %d\n", ctx.error_count);
    printf("log: %s\n", ctx.log);

    // --- Invalid transitions (should stay in same state) ---
    state = STATE_IDLE;
    enum State s2 = fsm_process(state, EVENT_PAUSE, &ctx);
    printf("\ninvalid: IDLE+PAUSE -> %s (should be IDLE)\n", state_names[s2]);

    return 0;
}
