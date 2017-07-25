Changelog
=========================

We have to kinds of updates. A major update occurs when y changes in x.y.z. A minor update when z changes.

Major updates can introduce code incompabilities with code that uses a previous version of this library. Minor updates are garantueed to only include bug fixes or upgrades of a few depedencies.

## Version 0.2

Removed the FromCookie trait. We use Rocket's private cookie system by default. So LoginStatus doens't need a type parameter for FromCookie anymore, see example/ for more information.

## Version 0.1.1

Upgraded to Rocket 0.2

## Version 0.1.0

Introduction of the library

