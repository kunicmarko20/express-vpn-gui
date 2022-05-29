ExpressVPN GUI
==============

A simple gui build on top of [ExpressVPN CLI](https://www.expressvpn.com/vpn-software/vpn-linux).

Basically, it just calls ExpressVPN CLI commands.
Shows if the VPN is on/off and allows to connect/disconnect to the VPN.

> Note: Tested only on Ubuntu 16.04+


Requirements
============

* [ExpressVPN CLI](https://www.expressvpn.com/vpn-software/vpn-linux)
* libappindicator installed


How to install
==============

* Download [express-vpn-gui](https://github.com/kunicmarko20/express-vpn-gui/releases/latest) binary
* Add binary to your `$PATH` and give it executable permissions if needed
* Run `express-vpn-gui install`

Now you should have a desktop entry and in your search you will be able to find ExpressVPN application.

If application is missing, just run `express-vpn-gui run -d`.


Screenshots
===========

Off:

![Off](doc/images/off.png)

On:

![On](doc/images/on.png)


Other Commands
==============

```bash
USAGE:
    express-vpn-gui <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help         Prints this message or the help of the given subcommand(s)
    install      Installs needed assets.
    run          Starts the Application
    uninstall    Removes everything

```

Development
===========

Building/installing from source can be done via a [Rust toolchain][rustup].

Additionally, you will need to install the development packages for GTK3 and libappindicator.  On Ubuntu 20.04
this can be done with:

```bash
$ sudo apt install libgtk-3-dev libappindicator3-dev
```

You can then build install using `cargo`:

```bash
$ cargo install --path .
```

[rustup]: https://rustup.rs/
