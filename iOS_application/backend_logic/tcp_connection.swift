import Socks 

class Run {
	let address = InternetAddress(hostname: "example.com", port: 80)

	do {
		let client = try TCPClient(address: address)

		try client.send("GET /\r\n\r\n")
		let returned = try client.read().makeString()
		try client.close()
		print("Received: \n\(returned)")
		} catch {
			print("Error \(error)")
		}

	}
}
