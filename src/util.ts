export function* permutations<T>(t: Iterable<T>, n: number): Generator<T[]> {
  const items = Array.from(t);
  const current: T[] = [];

  function* backtrack(depth: number): Generator<T[]> {
    if (depth === n) {
      yield [...current];
      return;
    }
    for (const item of items) {
      current.push(item);
      yield* backtrack(depth + 1);
      current.pop();
    }
  }

  yield* backtrack(0);
}

export namespace ty {
  export function keys<T>(t: T): Array<keyof T> {
    return Object.keys(t as object) as never;
  }

  export function values<T>(t: T): Array<T[keyof T]> {
    return Object.values(t as object) as never;
  }
}
