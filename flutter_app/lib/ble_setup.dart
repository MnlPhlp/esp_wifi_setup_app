import 'package:flutter/material.dart';
import 'package:esp_server_app/rust/api/ble.dart';

class BleSetup extends StatefulWidget {
  const BleSetup({Key? key}) : super(key: key);

  @override
  State<BleSetup> createState() => _BleSetupState();
}

class _BleSetupState extends State<BleSetup> {
  List<BleDevice> _devices = [];
  bool scanning = false;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Bluetooth setup')),
      body: Container(
        margin: const EdgeInsets.all(20),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.start,
          mainAxisSize: MainAxisSize.min,
          children: [
            Row(children: [
              TextButton(
                onPressed: scan,
                child: const Text("ble scan"),
              ),
              SizedBox(
                width: 10,
                height: 10,
                child:
                    scanning ? const CircularProgressIndicator() : Container(),
              )
            ]),
            Expanded(
              child: ListView.builder(
                itemCount: _devices.length,
                itemBuilder: (context, index) {
                  return ListTile(
                    leading: _devices[index].isConnected
                        ? const Icon(Icons.link)
                        : const SizedBox(width: 0, height: 0),
                    title: Text(_devices[index].name),
                    subtitle:
                        Text(formatAddress(address: _devices[index].address)),
                    onTap: () async {
                      await connect(address: _devices[index].address);
                      setState(() {
                        _devices = [];
                      });
                    },
                  );
                },
              ),
            ),
          ],
        ),
      ),
    );
  }

  @override
  void initState() {
    super.initState();
    scan();
  }

  void scan() {
    if (scanning) {
      return;
    }
    setState(() {
      scanning = true;
    });
    final devStream = startScan();
    devStream.listen((devices) {
      setState(() {
        _devices = devices;
      });
    }).onDone(
      () => setState(() {
        scanning = false;
      }),
    );
  }
}
