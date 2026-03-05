// ============================================================
// Canon C++98 — §11 Encapsulamiento (Access Control)
// ============================================================
// Intención: private/public controla quién puede acceder
// a los datos. Es verificación en COMPILACIÓN — cero
// costo en runtime. Los datos private siguen en el mismo
// struct en memoria.
//
// C++98 §11: "Members of a class defined with the keyword
// class are private by default."
// ============================================================

int printf(const char *format, ...);

// --- BankAccount: datos private, interface public ---
class BankAccount {
private:
    int balance;
    int transactions;
    int id;

public:
    BankAccount(int initial_id) : balance(0), transactions(0), id(initial_id) {}

    void deposit(int amount) {
        if (amount > 0) {
            balance = balance + amount;
            transactions = transactions + 1;
        }
    }

    int withdraw(int amount) {
        if (amount > 0 && amount <= balance) {
            balance = balance - amount;
            transactions = transactions + 1;
            return 1;
        }
        return 0;
    }

    int getBalance() { return balance; }
    int getTransactions() { return transactions; }
    int getId() { return id; }

    void print() {
        printf("  Account #%d: balance=%d txns=%d\n", id, balance, transactions);
    }
};

// --- Temperature: invariante mantenida ---
class Temperature {
private:
    int celsius;

public:
    Temperature() : celsius(0) {}
    Temperature(int c) : celsius(c) {}

    void setCelsius(int c) { celsius = c; }
    int getCelsius() { return celsius; }

    int getFahrenheit() {
        return celsius * 9 / 5 + 32;
    }

    int isFreezingC() { return celsius <= 0; }
    int isBoilingC() { return celsius >= 100; }
};

// --- Color: RGB encapsulated ---
class Color {
private:
    int r;
    int g;
    int b;

public:
    Color() : r(0), g(0), b(0) {}
    Color(int r, int g, int b) : r(r), g(g), b(b) {}

    int getRed() { return r; }
    int getGreen() { return g; }
    int getBlue() { return b; }

    void setRed(int v) { if (v >= 0 && v <= 255) r = v; }
    void setGreen(int v) { if (v >= 0 && v <= 255) g = v; }
    void setBlue(int v) { if (v >= 0 && v <= 255) b = v; }

    int toInt() {
        return (r << 16) | (g << 8) | b;
    }

    void print() {
        printf("  RGB(%d, %d, %d) = 0x%06X\n", r, g, b, toInt());
    }
};

int main() {
    printf("=== Canon C++98: Encapsulamiento ===\n\n");

    // --- BankAccount ---
    printf("BankAccount:\n");
    BankAccount acc(1001);
    acc.print();

    acc.deposit(1000);
    acc.deposit(500);
    printf("  After deposits:\n");
    acc.print();

    int ok = acc.withdraw(300);
    printf("  Withdraw 300: %s\n", ok ? "OK" : "FAIL");
    acc.print();

    int bad = acc.withdraw(9999);
    printf("  Withdraw 9999: %s\n", bad ? "OK" : "DENIED");
    acc.print();

    // --- Temperature ---
    printf("\nTemperature:\n");
    Temperature t(100);
    printf("  %d°C = %d°F\n", t.getCelsius(), t.getFahrenheit());
    printf("  boiling: %d\n", t.isBoilingC());

    t.setCelsius(0);
    printf("  %d°C = %d°F\n", t.getCelsius(), t.getFahrenheit());
    printf("  freezing: %d\n", t.isFreezingC());

    t.setCelsius(37);
    printf("  %d°C = %d°F\n", t.getCelsius(), t.getFahrenheit());

    // --- Color ---
    printf("\nColor:\n");
    Color red(255, 0, 0);
    red.print();

    Color green(0, 255, 0);
    green.print();

    Color custom(100, 150, 200);
    custom.print();

    // --- Verificación ---
    int pass = 0;
    int total = 0;

    total++; if (acc.getBalance() == 1200)    { pass++; } else { printf("FAIL: balance\n"); }
    total++; if (acc.getTransactions() == 3)   { pass++; } else { printf("FAIL: txns\n"); }
    total++; if (acc.getId() == 1001)          { pass++; } else { printf("FAIL: id\n"); }
    total++; if (t.getFahrenheit() == 98)      { pass++; } else { printf("FAIL: fahrenheit\n"); }
    total++; if (red.toInt() == 0xFF0000)      { pass++; } else { printf("FAIL: red\n"); }
    total++; if (green.toInt() == 0x00FF00)    { pass++; } else { printf("FAIL: green\n"); }

    printf("\n%d/%d passed\n", pass, total);
    return 0;
}
