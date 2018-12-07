import test from 'ava';
import day from '../days/1';


test('basic test 1', t => {
	t.is(day.run1("+1"), 1);
});

test('basic test 2', t => {
	t.is(day.run1("-2", 1), -1);
});

test('basic test 3', t => {
	t.is(day.run1("+3", -1), 2);
});

test('basic test 4', t => {
	t.is(day.run1("+1", 2), 3);
});

test('example 1.1.1', t => {
	t.is(day.run1("+1, +1, +1"), 3);
});

test('example 1.1.2', t => {
	t.is(day.run1("+1, +1, -2"), 0);
});

test('example 1.1.3', t => {
	t.is(day.run1("-1, -2, -3"), -6);
});

test('example 1.2.1', t => {
	t.is(day.run2("+1, -1"), 0);
});
test('example 1.2.2', t => {
	t.is(day.run2("+3, +3, +4, -2, -4"), 10);
});
test('example 1.2.3', t => {
	t.is(day.run2("-6, +3, +8, +5, -6"), 5);
});
test('example 1.2.4', t => {
	t.is(day.run2("+7, +7, -2, -7, -4"), 14);
});

/*
 * For example, if the device displays frequency changes of +1, -2, +3, +1, then starting from a frequency of zero, the following changes would occur:

Current frequency  0, change of +1; resulting frequency  1.
Current frequency  1, change of -2; resulting frequency -1.
Current frequency -1, change of +3; resulting frequency  2.
Current frequency  2, change of +1; resulting frequency  3.
In this example, the resulting frequency is 3.

Here are other example situations:

+1, +1, +1 results in  3
+1, +1, -2 results in  0
-1, -2, -3 results in -6
*/

/*
 * +1, -1 first reaches 0 twice.
+3, +3, +4, -2, -4 first reaches 10 twice.
-6, +3, +8, +5, -6 first reaches 5 twice.
+7, +7, -2, -7, -4 first reaches 14 twice.
*/
