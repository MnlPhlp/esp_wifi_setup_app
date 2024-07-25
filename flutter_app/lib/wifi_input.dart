import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:esp_server_app/rust/api/ble.dart';
import 'package:esp_server_app/state_handler.dart';

class WifiInput extends StatefulWidget {
  const WifiInput({super.key});

  @override
  State<WifiInput> createState() => _WifiInputState();
}

class _WifiInputState extends State<WifiInput> {
  TextEditingController ssid = TextEditingController();
  TextEditingController password = TextEditingController();

  @override
  Widget build(BuildContext context) {
    return Container(
      margin: const EdgeInsets.all(20),
      child: Column(
        children: [
          Row(children: [
            const Text("ESP IP:"),
            const SizedBox(width: 10),
            Consumer<StateHandler>(
              builder: (ctx, state, _) => Text(state.ip),
            ),
          ]),
          Row(children: [
            const SizedBox(width: 80, child: Text("SSID:")),
            Expanded(child: TextField(controller: ssid)),
          ]),
          Row(children: [
            const SizedBox(width: 80, child: Text("Password:")),
            Expanded(
              child: TextField(
                controller: password,
                obscureText: true,
              ),
            ),
          ]),
          TextButton(
            child: const Text("set wifi"),
            onPressed: () =>
                sendWifiData(ssid: ssid.text, password: password.text),
          )
        ],
      ),
    );
  }
}
