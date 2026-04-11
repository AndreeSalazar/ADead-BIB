#include <stdio.h>
#include <ctype.h>

int main() {
    int pass = 0, fail = 0;

    // isdigit
    if (isdigit('5')) { pass++; } else { fail++; printf("FAIL: isdigit('5')\n"); }
    if (!isdigit('x')) { pass++; } else { fail++; printf("FAIL: !isdigit('x')\n"); }

    // isalpha
    if (isalpha('A')) { pass++; } else { fail++; printf("FAIL: isalpha('A')\n"); }
    if (isalpha('z')) { pass++; } else { fail++; printf("FAIL: isalpha('z')\n"); }
    if (!isalpha('3')) { pass++; } else { fail++; printf("FAIL: !isalpha('3')\n"); }

    // isalnum
    if (isalnum('A')) { pass++; } else { fail++; printf("FAIL: isalnum('A')\n"); }
    if (isalnum('5')) { pass++; } else { fail++; printf("FAIL: isalnum('5')\n"); }
    if (!isalnum(' ')) { pass++; } else { fail++; printf("FAIL: !isalnum(' ')\n"); }

    // toupper / tolower
    if (toupper('a') == 'A') { pass++; } else { fail++; printf("FAIL: toupper\n"); }
    if (tolower('Z') == 'z') { pass++; } else { fail++; printf("FAIL: tolower\n"); }

    // isspace
    if (isspace(' ')) { pass++; } else { fail++; printf("FAIL: isspace(' ')\n"); }
    if (isspace('\t')) { pass++; } else { fail++; printf("FAIL: isspace('\\t')\n"); }
    if (!isspace('A')) { pass++; } else { fail++; printf("FAIL: !isspace('A')\n"); }

    // isupper / islower
    if (isupper('A')) { pass++; } else { fail++; printf("FAIL: isupper\n"); }
    if (islower('z')) { pass++; } else { fail++; printf("FAIL: islower\n"); }

    // isxdigit
    if (isxdigit('f')) { pass++; } else { fail++; printf("FAIL: isxdigit('f')\n"); }
    if (isxdigit('9')) { pass++; } else { fail++; printf("FAIL: isxdigit('9')\n"); }
    if (!isxdigit('g')) { pass++; } else { fail++; printf("FAIL: !isxdigit('g')\n"); }

    // isprint
    if (isprint('A')) { pass++; } else { fail++; printf("FAIL: isprint\n"); }

    printf("\n=== ctype_full: %d passed, %d failed ===\n", pass, fail);
    return fail;
}
