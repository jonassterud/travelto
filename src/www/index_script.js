function show_return_range() {
    let return_range = document.querySelector('#return-range')
    return_range.removeAttribute('hidden')
}

function hide_return_range() {
    let return_range = document.querySelector('#return-range')
    return_range.setAttribute('hidden', true)
}

function update_list(input, list_id) {
    if (!input.value) {
        return
    }

    fetch(`/locations?term=${input.value}`)
        .then((resp) => {
            return resp.json()
        })
        .then((data) => {
            let list = document.getElementById(list_id)
            list.innerHTML = ''

            data.forEach((suggestion, _) => {
                let element = document.createElement('option')
                element.innerText = suggestion.name
                element.value = suggestion.id

                list.appendChild(element)
            })
        })
        .catch((err) => console.error(err))
}
