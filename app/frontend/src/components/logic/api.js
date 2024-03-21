// API pro prihlasovani registraci a pod

const baseUrl = `http://127.0.0.1:8000`

export async function api(apiType = '', firstItem, secondItem) {
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

        case "getNets":
            try {
                response = await fetch(`${baseUrl}/api/net/${firstItem}`, {
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
export function getUrl(apiType="", id=0) {
  switch (apiType) {
    case "prefixes":
      return `/api/prefix/`
    case "hosts":
      return `/api/hosts/`
    case "getNets":
      return `/api/net/${id}`
    default:
      return `${baseUrl}`
    }
  }
export async function postApi(type='', inputData) {
  let response
  switch (type) {
    case "prefixes":
        try {
            response = await fetch(`${baseUrl}/api/prefixes/`, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json" },
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
