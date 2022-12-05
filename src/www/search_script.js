function sort() {
    let sort_by = document.querySelector('#sort-by').value
    let results = [...document.querySelectorAll('.result')].sort((a, b) => a.getAttribute(sort_by) - b.getAttribute(sort_by))

    results.forEach((e, i) => {
        e.style.order = i
    })
}
