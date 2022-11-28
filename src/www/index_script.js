function show_return_range() {
    let return_range = document.querySelector('#return-range')
    return_range.removeAttribute('hidden')
}

function hide_return_range() {
    let return_range = document.querySelector('#return-range')
    return_range.setAttribute('hidden', true)
}
