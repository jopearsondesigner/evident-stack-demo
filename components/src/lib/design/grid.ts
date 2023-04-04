export function setArrayLength(a: Array<any>, length: number): Array<any> {
  a.length = length;
  return a;
}

type PlacementContainerArray = Array<{ placements: Array<any> }>;

export function setAllPlacementArrayLengths(
  a: PlacementContainerArray,
  length: number
): PlacementContainerArray {
  a.forEach(pc => pc.placements.length = length);
  return a;
}

export function maxSparseArrayIndex(array: Array<any>): number {
  let max = 0;
  for (let i in array) {
    max = Math.max(max, parseInt(i));
  }
  return max
}

export function maxSparseArrayIndexInArray(maps: Array<Array<any>>): number {
  return Math.max(...maps.map(m => maxSparseArrayIndex(m)))
}
