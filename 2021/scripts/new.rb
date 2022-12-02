#!/usr/bin/env ruby
require 'fileutils'

def main(args)
	dirs = Dir['*'].filter{|f| File.directory?(f) and is_number?(f)}.map{|f| f.to_i}.sort
	newest = dirs.last
	newest = 0 if newest == nil
	target = "%02d" % (newest + 1)
	FileUtils.copy_entry("TEMPLATE", target)
	puts target
end

def is_number?(s)
	true if Float(s) rescue false
end

main(ARGV.dup)

