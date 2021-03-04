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

#[cfg_attr(any(target_os = "linux", target_os = "macos"),  global_allocator)]
#[cfg(any(target_os = "linux", target_os = "macos"))]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

struct BenchAdapter(tetsy_db::Db);

impl db_bench::Db for BenchAdapter {
	fn open(path: &std::path::Path) -> Self {
		BenchAdapter(tetsy_db::Db::with_columns(path, 1).unwrap())
	}

	fn get(&self, key: &db_bench::Key) -> Option<db_bench::Value> {
		self.0.get(0, key).unwrap()
	}

	fn commit<I: IntoIterator<Item=(db_bench::Key, Option<db_bench::Value>)>>(&self, tx: I) {
		self.0.commit(tx.into_iter().map(|(k, v)| (0, k, v))).unwrap()
	}
}

fn main() {
	db_bench::run::<BenchAdapter>();
}

