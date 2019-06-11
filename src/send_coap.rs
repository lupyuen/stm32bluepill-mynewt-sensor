//!  Send sensor data to a CoAP Server or a Collector Node.  The CoAP payload will be encoded as JSON
//!  for CoAP Server and CBOR for Collector Node.  The sensor data will be transmitted to 
//!  CoAP Server over WiFi via the ESP8266 transceiver, and to Collector Node via nRF24L01 transceiver.
//!
//!  This enables transmission of Sensor Data to a local Sensor Network (via nRF24L01)
//!  and to the internet (via ESP8266).  For sending to Collector Node we use raw temperature (integer) 
//!  instead of computed temperature (floating-point) to make the encoding simpler and faster.
//!
//!  Note that we are using a patched version of apps/my_sensor_app/src/vsscanf.c that
//!  fixes ESP8266 response parsing bugs.  The patched file must be present in that location.
//!  This is the Rust version of `https://github.com/lupyuen/stm32bluepill-mynewt-sensor/blob/rust/apps/my_sensor_app/OLDsrc/send_coap.c`

use cstr_core::CStr;                    //  Import string utilities from cstr_core library: https://crates.io/crates/cstr_core
use crate::base::*;                     //  Import base.rs for common declarations

macro_rules! calculate {
    (eval $e:expr) => {{
        {
            let val: usize = $e; // Force types to be integers
            //  println!("{} = {}", stringify!{$e}, val);
        }
    }};
}

fn test_macro() {
    calculate! {
        eval 1 + 2 // hehehe `eval` is _not_ a Rust keyword!
    }

    calculate! {
        eval (1 + 2) * (3 / 4)
    }
}

macro_rules! coap_root {
    ($blk:block) => {{
        {
            //  let val: usize = $e; // Force types to be integers
            //  println!("{} = {}", stringify!{$e}, val);
        }
    }};
}

macro_rules! coap_array {
    ($parent:ident, $key:ident, $blk:block) => {{
        {
            //  let val: usize = $e; // Force types to be integers
            //  println!("{} = {}", stringify!{$e}, val);
        }
    }};
}

macro_rules! coap_item_str {
    ($parent:ident, $key:expr, $val:expr) => {{
        {
            //  let val: usize = $e; // Force types to be integers
            //  println!("{} = {}", stringify!{$e}, val);
        }
    }};
}

fn test_macro2() {
    //  Compose the CoAP Payload in JSON using the CP macros.  Also works for CBOR.

    coap_item_str! (values, "device", device_id);  ////

    coap_array! (root, values, {  //  Create "values" as an array of items under the root
        coap_item_str! (values, "device", device_id);
        coap_item_str! (values, "node", node_id);
    }); ////

    let payload = coap_root! ({  //  Create the payload root
        coap_array! (root, values, {  //  Create "values" as an array of items under the root
            //  For Sensor Node: Set the Sensor Key and integer Sensor Value, e.g. { t: 2870 }
            ////TODO: coap_set_int_val! (root, val);

            //  Append to the "values" array:
            //    {"key":"device", "value":"0102030405060708090a0b0c0d0e0f10"},
            coap_item_str! (values, "device", device_id);

            //    {"key":"node", "value":"b3b4b5b6f1"},
            coap_item_str! (values, "node", node_id);

            //  If we are using raw temperature (integer) instead of computed temperature (float)...
            //  Append to the "values" array the Sensor Key and Sensor Value, depending on the value type:
            //    {"key":"t",   "value":2870} for raw temperature (integer)
            ////TODO: coap_item_int_val! (values, val);
            //    {"key":"tmp", "value":28.7} for computed temperature (float)
            //  coap_item_float_val! (values, val);

            //  If there are more sensor values, add them here with
            //  coap_item_int_val, coap_item_int, coap_item_uint, coap_item_float or coap_item_str

        }) //  Close the "values" array
    }); //  Close the payload root

}

///  TODO: Start the Network Task in the background.  The Network Task prepares the network drivers
///  (ESP8266 and nRF24L01) for transmitting sensor data messages.  
///  Connecting the ESP8266 to the WiFi access point may be slow so we do this in the background.
///  Also perform WiFi Geolocation if it is enabled.  Return 0 if successful.
pub fn start_network_task() -> Result<(), i32>  {  //  Returns an error code upon error.
//  pub fn start_network_task() -> i32  {
    console_print(b"start_network_task\n");
    test_macro();
    test_macro2();
    Ok(())
    //  0
}

///  TODO: Compose a CoAP message (CBOR or JSON) with the sensor value in `val` and transmit to the
///  Collector Node (if this is a Sensor Node) or to the CoAP Server (if this is a Collector Node
///  or Standalone Node).  
///  For Sensor Node or Standalone Node: sensor_node is the sensor name (`bme280_0` or `temp_stm32_0`)
///  For Collector Node: sensor_node is the Sensor Node Address of the Sensor Node that transmitted
///  the sensor data (like `b3b4b5b6f1`)
///  The message will be enqueued for transmission by the CoAP / OIC Background Task 
///  so this function will return without waiting for the message to be transmitted.  
///  Return 0 if successful, SYS_EAGAIN if network is not ready yet.
pub fn send_sensor_data(_val: *const SensorValue, _sensor_node: &'static CStr) -> i32 {
    console_print(b"send_sensor_data\n");

/*
    //  Compose the CoAP Payload in JSON using the CP macros.  Also works for CBOR.
    CP_ROOT({                     //  Create the payload root
        CP_ARRAY(root, values, {  //  Create "values" as an array of items under the root
            //  Append to the "values" array:
            //    {"key":"device", "value":"0102030405060708090a0b0c0d0e0f10"},
            CP_ITEM_STR(values, "device", device_id);

            //    {"key":"node", "value":"b3b4b5b6f1"},
            CP_ITEM_STR(values, "node", node_id);

#if MYNEWT_VAL(RAW_TEMP)  //  If we are using raw temperature (integer) instead of computed temperature (float)...
            //  Append to the "values" array the Sensor Key and Sensor Value, depending on the value type:
            //    {"key":"t",   "value":2870} for raw temperature (integer)
            CP_ITEM_INT_VAL(values, val);
#else       //    {"key":"tmp", "value":28.7} for computed temperature (float)
            CP_ITEM_FLOAT_VAL(values, val);
#endif  //  MYNEWT_VAL(RAW_TEMP)

            //  If there are more sensor values, add them here with
            //  CP_ITEM_VAL, CP_ITEM_INT, CP_ITEM_UINT, CP_ITEM_FLOAT or CP_ITEM_STR
            //  Check geolocate() for a more complex payload: apps/my_sensor_app/src/geolocate.c

        });                       //  End CP_ARRAY: Close the "values" array
    });                           //  End CP_ROOT:  Close the payload root

*/

    ;
    0
}

/*
static int send_sensor_data_to_server(struct sensor_value *val, const char *node_id) {
    //  Compose a CoAP JSON message with the Sensor Key (field name) and Value in val 
    //  and send to the CoAP server and URI.  The Sensor Value may be integer or float.
    //  For temperature, the Sensor Key is either "t" for raw temperature (integer, from 0 to 4095) 
    //  or "tmp" for computed temperature (float).
    //  The message will be enqueued for transmission by the CoAP / OIC 
    //  Background Task so this function will return without waiting for the message 
    //  to be transmitted.  Return 0 if successful, SYS_EAGAIN if network is not ready yet.

    //  For the CoAP server hosted at thethings.io, the CoAP payload should be encoded in JSON like this:
    //  {"values":[
    //    {"key":"device", "value":"0102030405060708090a0b0c0d0e0f10"},
    //    {"key":"tmp",    "value":28.7},
    //    {"key":"...",    "value":... },
    //    ... ]}
    assert(val);  assert(node_id);
    if (!network_is_ready) { return SYS_EAGAIN; }  //  If network is not ready, tell caller (Sensor Listener) to try later.
    const char *device_id = get_device_id();  assert(device_id);

    //  Start composing the CoAP Server message with the sensor data in the payload.  This will 
    //  block other tasks from composing and posting CoAP messages (through a semaphore).
    //  We only have 1 memory buffer for composing CoAP messages so it needs to be locked.
    int rc = init_server_post(NULL);  assert(rc != 0);

    //  Compose the CoAP Payload in JSON using the CP macros.  Also works for CBOR.
    CP_ROOT({                     //  Create the payload root
        CP_ARRAY(root, values, {  //  Create "values" as an array of items under the root
            //  Append to the "values" array:
            //    {"key":"device", "value":"0102030405060708090a0b0c0d0e0f10"},
            CP_ITEM_STR(values, "device", device_id);

            //    {"key":"node", "value":"b3b4b5b6f1"},
            CP_ITEM_STR(values, "node", node_id);

#if MYNEWT_VAL(RAW_TEMP)  //  If we are using raw temperature (integer) instead of computed temperature (float)...
            //  Append to the "values" array the Sensor Key and Sensor Value, depending on the value type:
            //    {"key":"t",   "value":2870} for raw temperature (integer)
            CP_ITEM_INT_VAL(values, val);
#else       //    {"key":"tmp", "value":28.7} for computed temperature (float)
            CP_ITEM_FLOAT_VAL(values, val);
#endif  //  MYNEWT_VAL(RAW_TEMP)

            //  If there are more sensor values, add them here with
            //  CP_ITEM_VAL, CP_ITEM_INT, CP_ITEM_UINT, CP_ITEM_FLOAT or CP_ITEM_STR
            //  Check geolocate() for a more complex payload: apps/my_sensor_app/src/geolocate.c

        });                       //  End CP_ARRAY: Close the "values" array
    });                           //  End CP_ROOT:  Close the payload root

    //  Post the CoAP Server message to the CoAP Background Task for transmission.  After posting the
    //  message to the background task, we release a semaphore that unblocks other requests
    //  to compose and post CoAP messages.
    rc = do_server_post();  assert(rc != 0);

    console_printf("NET view your sensor at \nhttps://blue-pill-geolocate.appspot.com?device=%s\n", device_id);
    //  console_printf("NET send data: tmp "); console_printfloat(tmp); console_printf("\n");  ////

    //  The CoAP Background Task will call oc_tx_ucast() in the ESP8266 driver to 
    //  transmit the message: libs/esp8266/src/transport.cpp
    return 0;
}

static int send_sensor_data_to_collector(struct sensor_value *val, const char *node_id) {
    //  Compose a CoAP CBOR message with the Sensor Key (field name) and Value in val and 
    //  transmit to the Collector Node.  The Sensor Value should be integer not float since
    //  we transmit integers only to the Collector Node.
    //  For temperature, the Sensor Key is "t" for raw temperature (integer, from 0 to 4095).
    //  The message will be enqueued for transmission by the CoAP / OIC 
    //  Background Task so this function will return without waiting for the message 
    //  to be transmitted.  Return 0 if successful, SYS_EAGAIN if network is not ready yet.
    //  The CoAP payload needs to be very compact (under 32 bytes) so it will be encoded in CBOR like this:
    //    { t: 2870 }
    assert(val);
    if (!network_is_ready) { return SYS_EAGAIN; }  //  If network is not ready, tell caller (Sensor Listener) to try later.

    //  Start composing the CoAP Collector message with the sensor data in the payload.  This will 
    //  block other tasks from composing and posting CoAP messages (through a semaphore).
    //  We only have 1 memory buffer for composing CoAP messages so it needs to be locked.
    int rc = init_collector_post();  assert(rc != 0);

    //  Compose the CoAP Payload in CBOR using the CBOR macros.
    CP_ROOT({  //  Create the payload root
        //  Set the Sensor Key and integer Sensor Value, e.g. { t: 2870 }
        CP_SET_INT_VAL(root, val);
    });  //  End CP_ROOT:  Close the payload root

    //  Post the CoAP Collector message to the CoAP Background Task for transmission.  After posting the
    //  message to the background task, we release a semaphore that unblocks other requests
    //  to compose and post CoAP messages.
    rc = do_collector_post();  assert(rc != 0);

    console_printf("NRF send to collector: rawtmp %d\n", val->int_val);  ////

    //  The CoAP Background Task will call oc_tx_ucast() in the nRF24L01 driver to 
    //  transmit the message: libs/nrf24l01/src/transport.cpp
    return 0;
}
*/