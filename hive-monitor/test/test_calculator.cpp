#include <unity.h>

void setUp(void) {
    // set stuff up here
}

void tearDown(void) {
    // clean stuff up here
}

int add(int a, int b) {
    return a + b;
}

void test_add() {
    TEST_ASSERT_EQUAL(add(1,2), 3);
}

int main(int argc, char **argv) {
    UNITY_BEGIN();
    RUN_TEST(test_add);
    UNITY_END();

    return 0;
}