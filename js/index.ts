void (async () => {
  const rust = await import('../rust/pkg')
  const txt = rust.break_lines('5 Reasons Drinking Milk On the Toilet Is Kind of a Game-Changer')
  console.log({ txt })
})()
