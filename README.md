# iprev

Reverse IP utility.

Easily reverse IPs in your workflow, e.g. for PTR or RBL
lookups.

## IPv4 Example

```bash
iprev 185.143.223.168 
```

Will produce

```
168.223.143.185.
```

## IPv6 Example

```bash
iprev 2a02:790:1:d::100:164
```

Will produce

```
4.6.1.0.0.0.1.0.0.0.0.0.0.0.0.0.d.0.0.0.1.0.0.0.0.9.7.0.2.0.a.2.
```

## RBL lookup example

Easily check popular blocklists for an IP.

```bash
dig +noall +answer $(iprev 185.143.223.168)zen.spamhaus.org
```

## PTR Lookup Example

Get the hostname from an IP address.

```bash
dig +short PTR $(iprev 52.203.88.233)in-addr.arpa
```

