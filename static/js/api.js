/**
 * @throws {"network"}
 * @param {"POST" | "GET"} method
 * @param {string} path
 * @param {Map<string, string>} body
 * @returns {Promise<any>}
**/
async function api(method, path, body = undefined) {
    let result;
    try {
        result = await fetch('/api' + path, {
            method: method,
            headers: {
                "Content-Type": "application/json"
            },
            body: body ? JSON.stringify(body) : undefined
        });
    } catch (e) {
        console.error(e);
        throw "network";
    }
    if (result.status != 200) {
        if (result.status == 401 || result.status == 403) {
            if (path == "/v1/auth/refreshSession") {
                window.location.href = "/login";
            }
            function getCookie(name) {
                var matches = document.cookie.match(new RegExp(
                    "(?:^|; )" + name.replace(/([\.$?*|{}\(\)\[\]\\\/\+^])/g, '\\$1') + "=([^;]*)"
                ))
                return matches ? decodeURIComponent(matches[1]) : undefined
            }

            localStorage.setItem("refreshToken", await api("POST", "/v1/auth/refreshSession", {
                accessToken: getCookie("jwt"),
                refreshToken: localStorage.getItem("refreshToken")
            }).refreshToken);

            return await api(method, path, body);
        }
        throw (await result.json()).error;
    }
    return result.json()
}