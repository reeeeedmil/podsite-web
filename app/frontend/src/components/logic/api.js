// API pro prihlasovani registraci a pod

const baseUrl = `http://localhost:8000`

async function api(apiType = '', firstItem, secondItem) {
    let response

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
export function getUrl(apiType="") {
  switch (apiType) {
    case "postPrefixes" | "prefixes":
      return `${baseUrl}/api/prefix/`
    case "postHosts" | "hosts":
      return `${baseUrl}/api/hosts/`
    default:
      return `${baseUrl}`
    }
  }
export async function postApi(type='', inputData) {
  let response
  console.log(JSON.stringify(inputData))
  switch (type) {
    case "prefixes":
        try {
            response = await fetch(`${baseUrl}/api/prefixes/`, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json"
                },
                body: JSON.stringify(inputData),
            });
            return response.json();
        } catch (error) {
            console.error("Error posting subnets: ", error);
            throw error;
        }

    case "hosts":
        try {
            response = await fetch(`${baseUrl}/api/hosts/`, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json"
                },
                body: JSON.stringify(inputData),
            });
            return response.json();
        } catch (error) {
            console.error("Error posting subnets: ", error);
            throw error;
        }
  }
}

export default api
