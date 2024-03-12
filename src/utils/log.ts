export function log(...args: any[]) {
  const res = args.reduce((acc, cur) => {
    if (isRef(cur))
      acc.push(cur.value)

    else if (typeof cur === 'function')
      acc.push(cur.toString())

    else
      acc.push(cur)

    return acc
  }, [] as any)

  // eslint-disable-next-line no-console
  console.log(...res)
}

export function wLog(...args: any[]) {
  watch(args, (value) => {
    log(...value)
  }, { deep: true })
}
