import test from 'ava';
import day from '../days/1';


test('basic test 1', t => {
	t.is(day.run("+1"), 1);
});

test('basic test 2', t => {
	t.is(day.run("-2", 1), -1);
});

test('basic test 3', t => {
	t.is(day.run("+3", -1), 2);
});

test('basic test 4', t => {
	t.is(day.run("+1", 2), 3);
});

test('example 1', t => {
	t.is(day.run("+1, +1, +1"), 3);
});

test('example 2', t => {
	t.is(day.run("+1, +1, -2"), 0);
});

test('example 3', t => {
	t.is(day.run("-1, -2, -3"), -6);
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
