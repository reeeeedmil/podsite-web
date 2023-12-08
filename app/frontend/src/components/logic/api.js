// API pro prihlasovani registraci a pod


async function api(apiType = '', firstItem, secondItem) {
    const baseUrl = `http://localhost:8000`
    let response
    let data

    switch (apiType) {
        case "getToken":
            data = { "username": firstItem, "password": secondItem }
            console.log(data)
            try {
                response = await fetch(`${baseUrl}/api-auth/`, {
                    method: "POST",
                    credentials: "include",
                    headers: {
                        "Content-Type": "application/json",
                    },
                    body: JSON.stringify(data)
                });
                return response.json();
            } catch (error) {
                console.error("Error fetching token:", error);
                throw error;
            }

        case "confirmAuth":
            try {
                response = await fetch(`${baseUrl}/api/confirm-auth/`, {
                    method: "POST",
                    credentials: "include",
                    headers: {
                        "Content-Type": "application/json",
                    },
                });
                return response.json();
            } catch (error) {
                console.error("Error confirming: ", error);
                throw error;
            }

        case "getUser":
            try {
                response = await fetch(`${baseUrl}/api/user/${firstItem}`, {
                    method: "GET",
                    credentials: "include",
                    headers: {
                        "Content-Type": "application/json",
                    },
                });
                return response.json();
            } catch (error) {
                console.error("Error getting user: ", error);
                throw error;
            }
        default:
            return `${baseUrl}`
    }
}

export default api
