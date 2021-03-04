// Copyright 2015-2020 Parity Technologies (UK) Ltd.
// This file is part of Tetsy.

// Tetsy is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Tetsy is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Tetsy.  If not, see <http://www.gnu.org/licenses/>.

pub type Key = [u8; 32];
pub type Value = Vec<u8>;

pub trait Db: Send + Sync + 'static {
	fn open(path: &std::path::Path) -> Self;
	fn get(&self, key: &Key) -> Option<Value>;
	fn commit<I: IntoIterator<Item=(Key, Option<Value>)>>(&self, tx: I);
}
