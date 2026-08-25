# 🏭 Industrial SCADA & Telemetry

Most web frameworks (like Express or Django) are built for HTTP CRUD applications. Vella is designed to run in highly volatile environments, including factory floors, racecars, and IoT edge servers.

## F1-Grade Telemetry
Vella ships with a hard real-time telemetry ingestion engine:
* **1000Hz UDP Listeners:** It can ingest data packets from physical sensors at over 1,000 times a second without blocking the main thread.
* **IPC Shared Memory:** For extreme low-latency environments (like interacting with C++ automotive computers), Vella can read and write data directly to system Shared Memory, completely bypassing the network stack.

## Industrial Protocols
Vella acts as a bridge between legacy factory hardware and modern web dashboards. It includes abstractions for:
* Modbus TCP / RTU
* OPC UA
* MQTT

## Swinging Door Compression
When ingesting thousands of sensor readings per second, database storage becomes incredibly expensive. Vella implements the **Swinging Door Compression** algorithm—a mathematical technique used in enterprise historians (like OSIsoft PI). 

It automatically calculates the slope of the incoming data and drops redundant data points on the fly, compressing your database storage by up to 90% while maintaining the exact shape of the sensor graph.
